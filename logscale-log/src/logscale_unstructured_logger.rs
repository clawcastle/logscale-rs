use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::options::{LoggerIngestPolicy, LoggerOptions};
use log::Log;
use logscale_client::{
    client::LogScaleClient,
    models::ingest::{UnstructuredLogEvent, UnstructuredLogsIngestRequest},
};

pub struct LogScaleUnstructuredLogger {
    log_ingester: UnstructuredLogIngester,
    options: LoggerOptions,
}

type PendingEvents = Arc<Mutex<Vec<UnstructuredLogEvent>>>;

struct UnstructuredLogIngester {
    client: LogScaleClient,
    pending_events: PendingEvents,
    ingest_policy: LoggerIngestPolicy,
}

impl UnstructuredLogIngester {
    pub fn new(client: LogScaleClient, ingest_policy: LoggerIngestPolicy) -> Self {
        Self {
            client,
            pending_events: Arc::from(Mutex::new(Vec::new())),
            ingest_policy,
        }
    }

    pub fn ingest_log_event(&self, log_event: UnstructuredLogEvent) {
        match self.ingest_policy {
            LoggerIngestPolicy::Immediately => {
                let client = self.client.clone();
                tokio::spawn(async move {
                    let request_content = [log_event];
                    let request = UnstructuredLogsIngestRequest::from_log_events(&request_content);

                    if client.ingest_unstructured(&[request]).await.is_err() {
                        eprintln!("An error occurred while trying to ingest logs to Falcon LogScale.");
                    }
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

                let mut events: Vec<UnstructuredLogEvent> = Vec::new();

                {
                    if let Ok(pending_events) = pending_events.lock() {
                        if pending_events.is_empty() {
                            continue;
                        }

                        events = pending_events.iter().cloned().collect();
                    }
                }

                let request = UnstructuredLogsIngestRequest::from_log_events(&events);
                if client.ingest_unstructured(&[request]).await.is_ok() {
                    if let Ok(mut pending_events) = pending_events.lock() {
                        pending_events.clear();
                    }
                } else {
                    eprintln!("An error occurred while trying to ingest logs to Falcon LogScale.");
                }
            }
        });
    }
}

impl LogScaleUnstructuredLogger {
    pub fn init(
        url: String,
        ingest_token: String,
        options: LoggerOptions,
    ) -> Result<(), Box<dyn Error>> {
        let mut logscale_logger = LogScaleUnstructuredLogger::create(&url, &ingest_token, options)?;

        if let LoggerIngestPolicy::Periodically(duration) = logscale_logger.options.ingest_policy {
            logscale_logger
                .log_ingester
                .start_background_ingest_job(duration);
        }

        log::set_boxed_logger(Box::from(logscale_logger))?;

        Ok(())
    }

    fn create(
        url: &str,
        ingest_token: &str,
        options: LoggerOptions,
    ) -> Result<Self, Box<dyn Error>> {
        let client = LogScaleClient::from_url(url, String::from(ingest_token))?;

        let log_ingester = UnstructuredLogIngester::new(client, options.ingest_policy);

        Ok(Self {
            log_ingester,
            options,
        })
    }
}

impl Log for LogScaleUnstructuredLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let log_event: UnstructuredLogEvent = record.args().to_string().into();

        self.log_ingester.ingest_log_event(log_event);
    }

    fn flush(&self) {}
}
