use anyhow::Result;
use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the local Jito Shredstream proxy.
    let mut client = ShredstreamProxyClient::connect("http://127.0.0.1:9999").await?;

    // Subscribe to the stream of reconstructed entries.
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await?
        .into_inner();

    // Print the slot from each streamed entry message.
    while let Some(entry) = stream.message().await? {
        println!("{}", entry.slot);
    }

    Ok(())
}
