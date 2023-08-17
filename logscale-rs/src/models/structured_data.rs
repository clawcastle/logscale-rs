use std::collections::HashMap;

use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct StructuredLogsIngestRequest<'a> {
    pub tags: HashMap<String, String>,
    pub events: &'a [StructuredLogEvent<'a>],
}

#[derive(Serialize)]
pub struct StructuredLogEvent<'a> {
    pub timestamp: u128,
    pub attributes: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
pub enum IngestStructuredDataError {
    FailedSendingRequest,
    RequestStatusCodeDidNotIndicateSuccess(StatusCode),
}
