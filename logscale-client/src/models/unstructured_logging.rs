use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct UnstructuredLogsIngestRequest<'a> {
    pub messages: &'a [&'a str],
    pub fields: HashMap<&'a str, &'a str>,
}

impl<'a> UnstructuredLogsIngestRequest<'a> {
    pub fn from_log_events(messages: &'a [&'a str]) -> Self {
        Self {
            messages,
            fields: HashMap::new(),
        }
    }
}
