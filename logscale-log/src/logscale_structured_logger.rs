use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use logscale_client::{
    client::LogScaleClient,
    models::ingest::{StructuredLogEvent, StructuredLogsIngestRequest},
};
use structured_logger::{Builder, Writer};

use crate::options::{LoggerIngestPolicy, LoggerOptions};

type PendingEvents = Arc<Mutex<Vec<StructuredLogEvent>>>;

struct StructuredLogIngester {
    client: LogScaleClient,
    pending_events: PendingEvents,
    ingest_policy: LoggerIngestPolicy,
}

impl StructuredLogIngester {
    pub fn new(client: LogScaleClient, ingest_policy: LoggerIngestPolicy) -> Self {
        Self {
            client,
            pending_events: Arc::from(Mutex::new(Vec::new())),
            ingest_policy,
        }
    }

    pub fn ingest_log_event(&self, log_event: StructuredLogEvent) {
        match self.ingest_policy {
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
                if let Ok(mut pending_events) = self.pending_events.lock() {
                    pending_events.push(log_event);
                }
            }
        }
    }

    fn start_background_ingest_job(&mut self, duration: Duration) {
        let client = self.client.clone();
        let pending_events = Arc::clone(&self.pending_events);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(duration);

            interval.tick().await;

            loop {
                interval.tick().await;

                let mut events: Vec<StructuredLogEvent> = Vec::new();

                {
                    if let Ok(pending_events) = pending_events.lock() {
                        if pending_events.is_empty() {
                            continue;
                        }

                        events = pending_events.iter().cloned().collect();
                    }
                }

                let request = StructuredLogsIngestRequest {
                    events: &events,
                    tags: HashMap::new(),
                };
                if client.ingest_structured(&[request]).await.is_ok() {
                    if let Ok(mut pending_events) = pending_events.lock() {
                        pending_events.clear();
                    }
                }
            }
        });
    }
}

pub struct LogScaleStructuredLogger {
    log_ingester: StructuredLogIngester,
    options: LoggerOptions,
}

impl LogScaleStructuredLogger {
    pub fn init(
        url: String,
        ingest_token: String,
        options: LoggerOptions,
    ) -> Result<(), Box<dyn Error>> {
        let mut logscale_logger = LogScaleStructuredLogger::create(&url, &ingest_token, options)?;

        if let LoggerIngestPolicy::Periodically(duration) = logscale_logger.options.ingest_policy {
            logscale_logger
                .log_ingester
                .start_background_ingest_job(duration);
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

        let log_ingester = StructuredLogIngester::new(client, options.ingest_policy);

        Ok(Self {
            log_ingester,
            options,
        })
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

        self.log_ingester.ingest_log_event(log_event);

        Ok(())
    }
}
