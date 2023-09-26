use std::{env, time::Duration};

use log::info;
use logscale_log::{
    logscale_unstructured_logger::LogScaleUnstructuredLogger, options::LoggerOptions,
};

#[derive(serde::Serialize)]
struct Nested {
    x: i32,
    y: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let ingest_token = args
        .get(1)
        .expect("Missing '--ingest-token' parameter.")
        .replace("--ingest-token=", "");

    LogScaleUnstructuredLogger::init(
        String::from("https://cloud.community.humio.com"),
        ingest_token,
        LoggerOptions::default(),
    )
    .unwrap();

    log::set_max_level(log::LevelFilter::Trace);

    // Loop to let the background sync task have time to do its thing.
    // Assumes that the provided ingest token has been set up with an "accesslog" parser to be able to parse the log line below.
    loop {
        info!("192.168.1.21 - user1 [26/Sep/2023:14:48:26 +0000] \"POST /humio/api/v1/ingest/elastic-bulk HTTP/1.1\" 200 0 \"-\" \"useragent\" 0.015 664 0.015");

        std::thread::sleep(Duration::from_secs(1));
    }
}
