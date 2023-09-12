use std::collections::HashMap;

use reqwest::StatusCode;
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
        Self { timestamp, attributes }
    }
}

#[derive(Debug)]
pub enum IngestStructuredDataError {
    FailedSendingRequest,
    RequestStatusCodeDidNotIndicateSuccess(StatusCode),
}
