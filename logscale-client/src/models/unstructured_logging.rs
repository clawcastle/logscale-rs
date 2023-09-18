use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct UnstructuredLogsIngestRequest<'a> {
    messages: &'a [&'a str],
    fields: HashMap<&'a str, &'a str>,
}
