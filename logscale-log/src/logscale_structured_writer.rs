use std::{
    collections::HashMap,
    error::Error,
    future,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use logscale_rs::{
    client::LogScaleClient,
    models::structured_data::{StructuredLogEvent, StructuredLogsIngestRequest},
};
use structured_logger::Writer;

pub struct LogScaleLogger<'a> {
    client: Mutex<LogScaleClient<'a>>,
}

impl<'a> LogScaleLogger<'a> {
    pub fn create(url: &'static str, ingest_token: &'static str) -> Result<Self, Box<dyn Error>> {
        let client = LogScaleClient::from_url(url, ingest_token)?;

        Ok(Self {
            client: Mutex::new(client),
        })
    }
}

impl<'a> Writer for LogScaleLogger<'a> {
    fn write_log(
        &self,
        value: &std::collections::BTreeMap<log::kv::Key, log::kv::Value>,
    ) -> Result<(), std::io::Error> {
        let client = self.client.lock();

        if let Ok(client) = client {
            let mut attributes = HashMap::with_capacity(value.len());

            for (key, val) in value {
                let val = val.to_borrowed_str().unwrap_or_default();
                attributes.insert(key.as_str(), val);
            }

            let now_unix_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let log_event = StructuredLogEvent {
                timestamp: now_unix_timestamp,
                attributes,
            };

            let request = StructuredLogsIngestRequest {
                tags: HashMap::new(),
                events: &[log_event],
            };
        }

        todo!()
    }
}
