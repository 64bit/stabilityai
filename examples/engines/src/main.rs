use std::error::Error;

use stabilityai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let response = client.engines().list().await?;
    println!("{:#?}", response);

    Ok(())
}
