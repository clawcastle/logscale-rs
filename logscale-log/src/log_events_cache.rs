use logscale_rs::models::structured_data::StructuredLogEvent;

pub struct LogsEventCache {
    cached_log_events: Vec<StructuredLogEvent>,
}

impl LogsEventCache {
    pub fn new() -> Self {
        Self {
            cached_log_events: Vec::new(),
        }
    }

    pub fn add_log_event(&mut self, log_event: StructuredLogEvent) {
        self.cached_log_events.push(log_event);

        println!("Size of cache: {}", self.cached_log_events.len());
    }
}
