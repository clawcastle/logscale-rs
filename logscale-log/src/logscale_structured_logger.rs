use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use logscale_rs::{client::LogScaleClient, models::structured_data::StructuredLogEvent};
use structured_logger::{Builder, Writer};

use crate::{ingest_job::start_background_ingest_job, log_events_cache::LogsEventCache};

pub struct LogScaleStructuredLogger {
    client: LogScaleClient,
    log_events_cache: Arc<Mutex<RefCell<LogsEventCache>>>,
}

impl LogScaleStructuredLogger {
    pub fn init(url: String, ingest_token: String) -> Result<(), Box<dyn Error>> {
        let logscale_logger = LogScaleStructuredLogger::create(&url, &ingest_token)?;

        logscale_logger.start();

        Builder::new()
            .with_default_writer(Box::from(logscale_logger))
            .init();

        Ok(())
    }

    pub fn create(url: &str, ingest_token: &str) -> Result<Self, Box<dyn Error>> {
        let client = LogScaleClient::from_url(url, String::from(ingest_token))?;

        Ok(Self {
            client,
            log_events_cache: Arc::from(Mutex::from(RefCell::new(LogsEventCache::new()))),
        })
    }

    fn start(&self) {
        let cloned_client = self.client.clone();
        let cloned_cache = Arc::clone(&self.log_events_cache);

        start_background_ingest_job(&cloned_client, cloned_cache);
    }
}

impl Writer for LogScaleStructuredLogger {
    fn write_log(
        &self,
        value: &std::collections::BTreeMap<log::kv::Key, log::kv::Value>,
    ) -> Result<(), std::io::Error> {
        let mut attributes = HashMap::with_capacity(value.len());

        for (key, val) in value {
            attributes.insert(key.to_string(), val.to_string());
        }

        let now_unix_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let log_event = StructuredLogEvent {
            timestamp: now_unix_timestamp,
            attributes: attributes.clone(),
        };

        if let Ok(mut cache) = self.log_events_cache.lock() {
            let cache = cache.get_mut();

            cache.add_log_event(log_event);
        }

        Ok(())
    }
}
