use logscale_client::models::structured_data::StructuredLogEvent;

pub struct LogsEventCache {
    cached_log_events: Vec<StructuredLogEvent>,
}

impl LogsEventCache {
    pub fn new() -> Self {
        Self {
            cached_log_events: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cached_log_events.is_empty()
    }

    pub fn get_log_events(&self) -> Vec<StructuredLogEvent> {
        self.cached_log_events.clone()
    }

    pub fn add_log_event(&mut self, log_event: StructuredLogEvent) {
        self.cached_log_events.push(log_event);
    }

    pub fn clear(&mut self) {
        self.cached_log_events.clear();
    }
}
