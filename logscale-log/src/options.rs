use std::time::Duration;

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            ingest_policy: LoggerIngestPolicy::Periodically(Duration::from_secs(5)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum LoggerIngestPolicy {
    Immediately,
    Periodically(Duration),
}

#[derive(Clone, Copy)]
pub struct LoggerOptions {
    pub ingest_policy: LoggerIngestPolicy,
}

impl LoggerOptions {
    pub fn new(ingest_policy: LoggerIngestPolicy) -> Self {
        Self { ingest_policy }
    }
}