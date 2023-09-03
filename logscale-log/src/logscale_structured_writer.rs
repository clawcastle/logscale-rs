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
use structured_logger::{Builder, Writer};

pub struct LogScaleLogger {
    client: Mutex<LogScaleClient>,
}

impl LogScaleLogger {
    pub fn init(url: String, ingest_token: String) -> Result<(), Box<dyn Error>> {
        let logscale_logger =
            LogScaleLogger::create("https://cloud.community.humio.com", &ingest_token)?;

        Builder::new()
            .with_default_writer(Box::from(logscale_logger))
            .init();

        Ok(())
    }

    pub fn create(url: &str, ingest_token: &str) -> Result<Self, Box<dyn Error>> {
        let client = LogScaleClient::from_url(String::from(url), String::from(ingest_token))?;

        Ok(Self {
            client: Mutex::new(client),
        })
    }
}

impl Writer for LogScaleLogger {
    fn write_log(
        &self,
        value: &std::collections::BTreeMap<log::kv::Key, log::kv::Value>,
    ) -> Result<(), std::io::Error> {
        let mut attributes = HashMap::with_capacity(value.len());

        for (key, val) in value {
            let val = val.to_borrowed_str().unwrap_or_default();
            attributes.insert(key.as_str(), val);
        }

        println!("{:?}", &attributes);

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

        todo!()
    }
}
