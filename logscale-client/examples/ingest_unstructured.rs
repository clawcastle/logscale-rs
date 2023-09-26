extern crate tokio;

use std::env;

use logscale_client::{
    client::LogScaleClient,
    models::ingest::{UnstructuredLogEvent, UnstructuredLogsIngestRequest},
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let ingest_token = args
        .get(1)
        .expect("Missing '--ingest-token' parameter.")
        .replace("--ingest-token=", "");

    let logscale_client =
        LogScaleClient::from_url("https://cloud.community.humio.com", ingest_token).unwrap();

    let log_messages = ["192.168.1.21 - user1 [18/Sep/2023:12:48:26 +0000] \"POST /humio/api/v1/ingest/elastic-bulk HTTP/1.1\" 200 0 \"-\" \"useragent\" 0.015 664 0.015", "192.168.1.21 - user1 [18/Sep/2023:12:48:26 +0000] \"POST /humio/api/v1/ingest/elastic-bulk HTTP/1.1\" 200 0 \"-\" \"useragent\" 0.015 664 0.015", "192.168.1.21 - user1 [18/Sep/2023:12:48:26 +0000] \"POST /humio/api/v1/ingest/elastic-bulk HTTP/1.1\" 200 0 \"-\" \"useragent\" 0.015 664 0.015", "192.168.1.21 - user1 [18/Sep/2023:12:48:26 +0000] \"POST /humio/api/v1/ingest/elastic-bulk HTTP/1.1\" 200 0 \"-\" \"useragent\" 0.015 664 0.015"];
    let events: Vec<UnstructuredLogEvent> = log_messages
        .into_iter()
        .map(|msg| msg.to_string().into())
        .collect();

    let request = UnstructuredLogsIngestRequest::from_log_events(&events);

    logscale_client
        .ingest_unstructured(&[request])
        .await
        .unwrap();
}
