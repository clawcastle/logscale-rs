use thiserror::Error;


#[derive(Debug, Error)]
pub enum LogScaleError {
    #[error("An error occurred while trying to send the request to LogScale.")]
    ConnectionError,
    #[error("LogScale returned a message with an HTTP status code indicating an error. Status code: {0}.")]
    ErrorWithStatusCode(u16),
}

