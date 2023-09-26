extern crate tokio;

use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

use logscale_client::{
    client::LogScaleClient,
    models::ingest::{StructuredLogEvent, StructuredLogsIngestRequest},
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

    let now_unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let attributes = serde_json::to_value(HashMap::<String, String>::new()).unwrap();

    let events: Vec<StructuredLogEvent> =
        vec![StructuredLogEvent::new(now_unix_timestamp, attributes)];

    let request = StructuredLogsIngestRequest {
        events: &events,
        tags: HashMap::new(),
    };

    logscale_client.ingest_structured(&[request]).await.unwrap();
}
