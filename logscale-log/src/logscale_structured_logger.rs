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

use crate::{ingest_job::start_background_ingest_job, log_events_cache::LogsEventCache};

#[derive(Clone, Copy)]
pub enum StructuredLoggerIngestPolicy {
    Immediately,
    Periodically(Duration),
}

#[derive(Clone, Copy)]
pub struct StructuredLoggerOptions {
    pub ingest_policy: StructuredLoggerIngestPolicy,
}

impl StructuredLoggerOptions {
    pub fn new(ingest_policy: StructuredLoggerIngestPolicy) -> Self {
        Self { ingest_policy }
    }
}

impl Default for StructuredLoggerOptions {
    fn default() -> Self {
        Self {
            ingest_policy: StructuredLoggerIngestPolicy::Periodically(Duration::from_secs(5)),
        }
    }
}

pub struct LogScaleStructuredLogger {
    client: LogScaleClient,
    options: StructuredLoggerOptions,
    log_events_cache: Arc<Mutex<RefCell<LogsEventCache>>>,
}

impl LogScaleStructuredLogger {
    pub fn init(
        url: String,
        ingest_token: String,
        options: StructuredLoggerOptions,
    ) -> Result<(), Box<dyn Error>> {
        let logscale_logger = LogScaleStructuredLogger::create(&url, &ingest_token, options)?;

        if let StructuredLoggerIngestPolicy::Periodically(duration) =
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
        options: StructuredLoggerOptions,
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
            attributes,
        };

        match self.options.ingest_policy {
            StructuredLoggerIngestPolicy::Immediately => {
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
            StructuredLoggerIngestPolicy::Periodically(_) => {
                if let Ok(mut cache) = self.log_events_cache.lock() {
                    let cache = cache.get_mut();

                    cache.add_log_event(log_event);
                }
            }
        }

        Ok(())
    }
}
