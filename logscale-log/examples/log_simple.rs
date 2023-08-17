use log::info;
use logscale_log::logscale_structured_writer::LogScaleLogger;
fn main() {
    let logscale_logger = LogScaleLogger;

    log::set_boxed_logger(Box::new(logscale_logger)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let param = 42;
    info!(param = format!("{}", param); "Log with param");
}
