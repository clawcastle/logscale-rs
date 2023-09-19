use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use logscale_client::{
    client::LogScaleClient,
    models::structured_logging::{StructuredLogEvent, StructuredLogsIngestRequest},
};
use structured_logger::{Builder, Writer};

use crate::{ingest_job::start_background_ingest_job, log_events_cache::LogsEventCache, options::{LoggerIngestPolicy, LoggerOptions}};

pub struct LogScaleStructuredLogger {
    client: LogScaleClient,
    options: LoggerOptions,
    log_events_cache: Arc<Mutex<RefCell<LogsEventCache>>>,
}

impl LogScaleStructuredLogger {
    pub fn init(
        url: String,
        ingest_token: String,
        options: LoggerOptions,
    ) -> Result<(), Box<dyn Error>> {
        let logscale_logger = LogScaleStructuredLogger::create(&url, &ingest_token, options)?;

        if let LoggerIngestPolicy::Periodically(duration) =
            logscale_logger.options.ingest_policy
        {
            logscale_logger.start_periodic_sync(duration);
        }

        Builder::new()
            .with_default_writer(Box::from(logscale_logger))
            .init();

        Ok(())
    }

    fn create(
        url: &str,
        ingest_token: &str,
        options: LoggerOptions,
    ) -> Result<Self, Box<dyn Error>> {
        let client = LogScaleClient::from_url(url, String::from(ingest_token))?;

        Ok(Self {
            client,
            options,
            log_events_cache: Arc::from(Mutex::from(RefCell::new(LogsEventCache::new()))),
        })
    }

    fn start_periodic_sync(&self, duration: Duration) {
        let cloned_client = self.client.clone();
        let cloned_cache = Arc::clone(&self.log_events_cache);

        start_background_ingest_job(duration, &cloned_client, cloned_cache);
    }
}

impl Writer for LogScaleStructuredLogger {
    fn write_log(
        &self,
        value: &std::collections::BTreeMap<log::kv::Key, log::kv::Value>,
    ) -> Result<(), std::io::Error> {
        let attributes = serde_json::to_value(value)?;

        let now_unix_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let log_event = StructuredLogEvent::new(now_unix_timestamp, attributes);
        
        match self.options.ingest_policy {
            LoggerIngestPolicy::Immediately => {
                let client = self.client.clone();

                tokio::spawn(async move {
                    let _ = client
                        .ingest_structured(&[StructuredLogsIngestRequest {
                            tags: HashMap::new(),
                            events: &[log_event],
                        }])
                        .await;
                });
            }
            LoggerIngestPolicy::Periodically(_) => {
                if let Ok(mut cache) = self.log_events_cache.lock() {
                    let cache = cache.get_mut();

                    cache.add_log_event(log_event);
                }
            }
        }

        Ok(())
    }
}
