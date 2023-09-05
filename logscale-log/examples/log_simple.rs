use std::{env, time::Duration};

use log::info;
use logscale_log::logscale_structured_logger::LogScaleStructuredLogger;

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
    )
    .unwrap();

    log::set_max_level(log::LevelFilter::Trace);

    let param = 42;
    info!(param = format!("{}", param); "Log with param");

    // Loop to let the background sync task have time to do its thing.
    loop {
        std::thread::sleep(Duration::from_secs(5));
    }
}
