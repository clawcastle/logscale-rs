use std::error::Error;

use reqwest::{Client, StatusCode, Url};

use crate::models::{
    errors::LogScaleError, structured_logging::StructuredLogsIngestRequest,
    unstructured_logging::UnstructuredLogsIngestRequest,
};

#[derive(Clone)]
pub struct LogScaleClient {
    logscale_url: Url,
    ingest_token: String,
    http_client: Client,
    ingest_token_header_value: String,
}

impl LogScaleClient {
    pub fn from_url(logscale_url: &str, ingest_token: String) -> Result<Self, Box<dyn Error>> {
        let url = Url::parse(logscale_url)?;

        Ok(Self {
            logscale_url: url,
            ingest_token: ingest_token.clone(),
            http_client: Client::default(),
            ingest_token_header_value: format!("Bearer {}", &ingest_token),
        })
    }

    pub async fn ingest_structured<'a>(
        &self,
        request: &[StructuredLogsIngestRequest<'a>],
    ) -> Result<(), LogScaleError> {
        let url = self
            .logscale_url
            .join("api/v1/ingest/humio-structured")
            .unwrap();

        if let Ok(response) = self
            .http_client
            .post(url)
            .header("Authorization", &self.ingest_token_header_value)
            .json(&request)
            .send()
            .await
        {
            let response_status_code = response.status();

            if response_status_code == StatusCode::OK {
                Ok(())
            } else {
                Err(LogScaleError::ErrorWithStatusCode(
                    response_status_code.into(),
                ))
            }
        } else {
            Err(LogScaleError::ConnectionError)
        }
    }

    pub async fn ingest_unstructured<'a>(
        &self,
        request: &UnstructuredLogsIngestRequest<'a>,
    ) -> Result<(), LogScaleError> {
        let url = self
            .logscale_url
            .join("api/v1/ingest/humio-unstructured")
            .unwrap();

        if let Ok(response) = self
            .http_client
            .post(url)
            .header("Authorization", &self.ingest_token_header_value)
            .json(&request)
            .send()
            .await
        {
            let response_status_code = response.status();

            if response_status_code == StatusCode::OK {
                Ok(())
            } else {
                Err(LogScaleError::ErrorWithStatusCode(
                    response_status_code.into(),
                ))
            }
        } else {
            Err(LogScaleError::ConnectionError)
        }
    }
}

unsafe impl Send for LogScaleClient {}
unsafe impl Sync for LogScaleClient {}
