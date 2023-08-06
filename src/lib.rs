use std::{collections::HashMap, error::Error, fmt, path::Display, string::ParseError};

use reqwest::{Client, StatusCode};
use serde::Serialize;

struct LogScaleClient {
    logscale_url: reqwest::Url,
    ingest_token: &'static str,
    http_client: Client,
    ingest_token_header_value: String,
}

impl LogScaleClient {
    pub fn from_url(
        logscale_url: &'static str,
        ingest_token: &'static str,
    ) -> Result<Self, Box<dyn Error>> {
        let url = reqwest::Url::parse(logscale_url)?;

        Ok(Self {
            logscale_url: url,
            ingest_token,
            http_client: Client::default(),
            ingest_token_header_value: format!("Bearer {}", ingest_token),
        })
    }

    async fn ingest_structured(
        &self,
        request: &[StructuredLogsIngestRequest],
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
struct StructuredLogsIngestRequest {
    tags: HashMap<String, String>,
    events: Vec<StructuredLogEvent>,
}

#[derive(Serialize)]
struct StructuredLogEvent {
    timestamp: String,
    attributes: HashMap<String, String>,
}

enum IngestStructuredDataError {
    FailedSendingRequest,
    RequestStatusCodeDidNotIndicateSuccess(StatusCode),
}
