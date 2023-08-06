extern crate tokio;

use logscale_rs::LogScaleClient;

#[tokio::main]
async fn main() {
    let logscale_client =
        LogScaleClient::from_url("https://cloud.community.humio.com", "INGEST_TOKEN").unwrap();
}
