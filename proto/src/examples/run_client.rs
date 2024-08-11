use proto::examples::dummy_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client: Client = Client::new().await?;

    let response: tonic::Response<()> = client.send_dummy_span_creation_request().await?;
    println!("RESPONSE={:?}", response);

    let response: tonic::Response<()> = client.send_dummy_test_execution_request().await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}