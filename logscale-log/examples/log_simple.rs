use std::env;

use log::info;
use logscale_log::logscale_structured_writer::LogScaleLogger;
use structured_logger::Builder;
fn main() {
    let args: Vec<String> = env::args().collect();

    let ingest_token = args
        .get(1)
        .expect("Missing '--ingest-token' parameter.")
        .replace("--ingest-token=", "");

    let logscale_logger =
        LogScaleLogger::create("https://cloud.community.humio.com", &ingest_token).unwrap();

    Builder::new()
        .with_default_writer(Box::from(logscale_logger))
        .init();

    // log::set_boxed_logger(Box::new(logscale_logger)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let param = 42;
    info!(param = format!("{}", param); "Log with param");
}
