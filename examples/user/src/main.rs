use std::error::Error;

use stabilityai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let response = client.user().account().await?;
    println!("{:#?}", response);

    let response = client.user().balance().await?;
    println!("{:#?}", response);

    Ok(())
}
