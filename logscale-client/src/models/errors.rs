#[derive(Debug)]
pub enum LogScaleError {
    ConnectionError,
    ErrorWithStatusCode(u16),
}
