use std::{env, time::Duration};

use log::info;
use logscale_log::logscale_logger::LogScaleLogger;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let ingest_token = args
        .get(1)
        .expect("Missing '--ingest-token' parameter.")
        .replace("--ingest-token=", "");

    LogScaleLogger::init(
        String::from("https://cloud.community.humio.com"),
        ingest_token,
    )
    .unwrap();

    log::set_max_level(log::LevelFilter::Trace);

    let param = 42;
    info!(param = format!("{}", param); "Log with param");

    loop {
        std::thread::sleep(Duration::from_secs(5));
    }
}
