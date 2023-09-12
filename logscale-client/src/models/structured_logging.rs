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

#[derive(Debug)]
pub enum IngestStructuredDataError {
    FailedSendingRequest,
    RequestStatusCodeDidNotIndicateSuccess(StatusCode),
}
