extern crate tokio;

use std::{collections::HashMap, env, vec};

use logscale_rs::{LogScaleClient, StructuredLogEvent, StructuredLogsIngestRequest};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);
    let ingest_token = args[1].replace("--ingest-token=", "");
    println!("{}", &ingest_token);

    let logscale_client =
        LogScaleClient::from_url("https://cloud.community.humio.com", &ingest_token).unwrap();

    let events: Vec<StructuredLogEvent> = vec![StructuredLogEvent {
        timestamp: "2023-08-06T12:00:00+02:00".to_string(),
        attributes: HashMap::new(),
    }];

    let request = StructuredLogsIngestRequest {
        events: &events,
        tags: HashMap::new(),
    };

    logscale_client.ingest_structured(&[request]).await.unwrap();
}
