use std::collections::HashMap;

use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct StructuredLogsIngestRequest<'a> {
    pub tags: HashMap<String, String>,
    pub events: &'a [StructuredLogEvent],
}

#[derive(Serialize)]
pub struct StructuredLogEvent {
    pub timestamp: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub enum IngestStructuredDataError {
    FailedSendingRequest,
    RequestStatusCodeDidNotIndicateSuccess(StatusCode),
}
