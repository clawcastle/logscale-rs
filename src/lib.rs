use std::{collections::HashMap, error::Error};

use reqwest::{Client, StatusCode};
use serde::Serialize;

pub struct LogScaleClient<'a> {
    logscale_url: reqwest::Url,
    ingest_token: &'a str,
    http_client: Client,
    ingest_token_header_value: String,
}

impl<'a> LogScaleClient<'a> {
    pub fn from_url(
        logscale_url: &'static str,
        ingest_token: &'a str,
    ) -> Result<Self, Box<dyn Error>> {
        let url = reqwest::Url::parse(logscale_url)?;

        Ok(Self {
            logscale_url: url,
            ingest_token,
            http_client: Client::default(),
            ingest_token_header_value: format!("Bearer {}", ingest_token),
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

#[derive(Serialize)]
pub struct StructuredLogsIngestRequest<'a> {
    pub tags: HashMap<String, String>,
    pub events: &'a [StructuredLogEvent],
}

#[derive(Serialize)]
pub struct StructuredLogEvent {
    pub timestamp: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub enum IngestStructuredDataError {
    FailedSendingRequest,
    RequestStatusCodeDidNotIndicateSuccess(StatusCode),
}
