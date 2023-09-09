use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use logscale_rs::{
    client::LogScaleClient,
    models::structured_data::{StructuredLogEvent, StructuredLogsIngestRequest},
};

use crate::log_events_cache::LogsEventCache;

pub fn start_background_ingest_job(
    duration: Duration,
    logscale_client: &LogScaleClient,
    cache: Arc<Mutex<RefCell<LogsEventCache>>>,
) {
    let cloned_client = logscale_client.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(duration);

        interval.tick().await;

        loop {
            interval.tick().await;

            let mut events: Vec<StructuredLogEvent> = Vec::new();

            {
                if let Ok(c) = cache.lock() {
                    if let Ok(c) = c.try_borrow() {
                        if c.is_empty() {
                            continue;
                        }
    
                        events = c.get_log_events();
                    }
                }
            }

            if cloned_client
                .ingest_structured(&[StructuredLogsIngestRequest {
                    tags: HashMap::new(),
                    events: &events,
                }])
                .await
                .is_ok()
            {
                if let Ok(mut c) = cache.lock() {
                    let c = c.get_mut();
                    c.clear();
                }
            }
        }
    });
}
