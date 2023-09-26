use std::collections::HashMap;

use serde::Serialize;
use serde_json::value::Value as JsonValue;

#[derive(Serialize)]
pub struct StructuredLogsIngestRequest<'a> {
    pub tags: HashMap<String, String>,
    pub events: &'a [StructuredLogEvent],
}

#[derive(Serialize, Clone)]
pub struct StructuredLogEvent {
    pub timestamp: u128,
    pub attributes: JsonValue,
}

impl StructuredLogEvent {
    pub fn new(timestamp: u128, attributes: JsonValue) -> Self {
        Self {
            timestamp,
            attributes,
        }
    }
}

#[derive(Clone)]
pub struct UnstructuredLogEvent {
    message: String,
}

impl Serialize for UnstructuredLogEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.message)
    }
}

impl From<String> for UnstructuredLogEvent {
    fn from(value: String) -> Self {
        UnstructuredLogEvent { message: value }
    }
}

#[derive(Serialize)]
pub struct UnstructuredLogsIngestRequest<'a> {
    pub messages: &'a [UnstructuredLogEvent],
    pub fields: HashMap<&'a str, &'a str>,
}

impl<'a> UnstructuredLogsIngestRequest<'a> {
    pub fn from_log_events(log_events: &'a [UnstructuredLogEvent]) -> Self {
        Self {
            messages: log_events,
            fields: HashMap::new(),
        }
    }
}

pub trait LogScaleLogEvent: Clone + Serialize {}

impl LogScaleLogEvent for StructuredLogEvent {}
impl LogScaleLogEvent for UnstructuredLogEvent {}
