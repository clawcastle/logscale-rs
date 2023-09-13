use std::{env, time::Duration};

use log::{info, error};
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

    let nested = Nested {x: 42, y: String::from("some string parameter.")};
    let mut count = 0;
    
    // Loop to let the background sync task have time to do its thing.
    loop {
        info!("Plain log only with text, no variables");

        error!("You can of course also use any other log level, like error.");

        info!(count = format!("{}", count); "You can also add structured values to your logs, which will automatically be included as attributes in your LogScale logs.");

        info!(target: "api",
            method = "GET", 
            path = "/hello", 
            status = 200_u16, 
            elapsed = 10_u64, 
            nested = log::as_serde!(nested); 
            "You can also create logs with multiple structured values, and even nested JSON values.",);

        std::thread::sleep(Duration::from_secs(1));
        
        count += 1;
    }
}
