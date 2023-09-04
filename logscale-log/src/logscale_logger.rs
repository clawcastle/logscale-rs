use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use logscale_rs::{
    client::LogScaleClient,
    models::structured_data::{StructuredLogEvent, StructuredLogsIngestRequest},
};
use structured_logger::{Builder, Writer};

use crate::log_events_cache::LogsEventCache;

pub struct LogScaleLogger {
    client: LogScaleClient,
    log_events_cache: Arc<Mutex<RefCell<LogsEventCache>>>,
}

impl LogScaleLogger {
    pub fn init(url: String, ingest_token: String) -> Result<(), Box<dyn Error>> {
        let logscale_logger = LogScaleLogger::create(&url, &ingest_token)?;

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

    async fn flush_log_events(client: LogScaleClient, cache: Arc<Mutex<RefCell<LogsEventCache>>>) {
        if let Ok(mut cache) = cache.lock() {
            let c = cache.get_mut();

            if c.is_empty() {
                return;
            }

            let events = c.get_log_events();

            if (client
                .ingest_structured(&[StructuredLogsIngestRequest {
                    tags: HashMap::new(),
                    events: &events,
                }])
                .await)
                .is_ok()
            {
                c.clear();
            }
        }
    }
}

impl Writer for LogScaleLogger {
    fn write_log(
        &self,
        value: &std::collections::BTreeMap<log::kv::Key, log::kv::Value>,
    ) -> Result<(), std::io::Error> {
        let mut attributes = HashMap::with_capacity(value.len());

        for (key, val) in value {
            attributes.insert(key.to_string(), val.to_string());
        }

        println!("{:?}", &attributes);

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
