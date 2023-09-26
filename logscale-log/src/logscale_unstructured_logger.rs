use std::{
    cell::RefCell,
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};

use log::Log;
use logscale_client::{client::LogScaleClient, models::ingest::UnstructuredLogsIngestRequest};

use crate::{
    ingest_job::start_background_ingest_job,
    log_events_cache::LogsEventCache,
    options::{LoggerIngestPolicy, LoggerOptions},
};

pub struct LogScaleUnstructuredLogger {
    client: LogScaleClient,
    options: LoggerOptions,
    log_events_cache: Arc<Mutex<RefCell<LogsEventCache>>>,
}

impl LogScaleUnstructuredLogger {
    pub fn init(
        url: String,
        ingest_token: String,
        options: LoggerOptions,
    ) -> Result<(), Box<dyn Error>> {
        let logscale_logger = LogScaleUnstructuredLogger::create(&url, &ingest_token, options)?;

        if let LoggerIngestPolicy::Periodically(duration) = logscale_logger.options.ingest_policy {
            logscale_logger.start_periodic_sync(duration);
        }

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

impl Log for LogScaleUnstructuredLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        match self.options.ingest_policy {
            LoggerIngestPolicy::Immediately => {
                if let Some(log_message) = record.args().as_str() {
                    let client = self.client.clone();
                    tokio::spawn(async move {
                        let request_content = [log_message];
                        let request =
                            UnstructuredLogsIngestRequest::from_log_events(&request_content);

                        let _ = client.ingest_unstructured(&[request]).await;
                    });
                }
            }
            LoggerIngestPolicy::Periodically(_) => todo!(),
        }
    }

    fn flush(&self) {}
}
