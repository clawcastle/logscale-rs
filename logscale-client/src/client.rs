use std::error::Error;

use reqwest::{Client, StatusCode, Url};

use crate::models::structured_data::{IngestStructuredDataError, StructuredLogsIngestRequest};

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

    pub async fn ingest_structured<'b>(
        &self,
        request: &[StructuredLogsIngestRequest<'b>],
    ) -> Result<(), IngestStructuredDataError> {
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
                Err(
                    IngestStructuredDataError::RequestStatusCodeDidNotIndicateSuccess(
                        response_status_code,
                    ),
                )
            }
        } else {
            Err(IngestStructuredDataError::FailedSendingRequest)
        }
    }
}

unsafe impl Send for LogScaleClient {}
unsafe impl Sync for LogScaleClient {}
