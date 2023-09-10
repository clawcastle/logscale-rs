use std::{env, time::Duration, collections::HashMap};

use log::info;
use logscale_log::logscale_structured_logger::{LogScaleStructuredLogger, StructuredLoggerOptions};

#[derive(serde::Serialize)]
struct Nested {
    x: i32,
    y: String
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let ingest_token = args
        .get(1)
        .expect("Missing '--ingest-token' parameter.")
        .replace("--ingest-token=", "");

    LogScaleStructuredLogger::init(
        String::from("https://cloud.community.humio.com"),
        ingest_token,
        StructuredLoggerOptions::default()
    )
    .unwrap();

    log::set_max_level(log::LevelFilter::Trace);

    let mut count = 0;

    let nested = Nested {x: 42, y: String::from("jaaa")};

    
    // Loop to let the background sync task have time to do its thing.
    loop {
        info!(kv = log::as_serde!(nested); "Log with kv parameter");
        count += 1;
        std::thread::sleep(Duration::from_secs(1));
    }
}
