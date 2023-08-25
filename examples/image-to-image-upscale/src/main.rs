use std::error::Error;

use stabilityai::{
    error::StabilityAIError,
    types::{LatentUpscalerUpscaleRequestBodyArgs, RealESRGANUpscaleRequestBodyArgs},
    Client,
};

async fn real_esrgan_upscale(client: &Client) -> Result<(), StabilityAIError> {
    println!("Requesting upscale with esrgan-v1-x2plus");

    let request = RealESRGANUpscaleRequestBodyArgs::default()
        .image("./image-data/Rabindranath_with_Einstein.jpeg")
        .build()?;

    let artifacts = client
        .generate("esrgan-v1-x2plus")
        .image_to_image_upscale(request)
        .await?;

    let paths = artifacts.save("./data/esrgan-upscale").await?;

    paths
        .iter()
        .for_each(|path| println!("Image saved at {}", path.display()));

    Ok(())
}

async fn latent_upscaler_upscale(client: &Client) -> Result<(), StabilityAIError> {
    println!("Requesting upscale with stable-diffusion-x4-latent-upscaler");

    let request = LatentUpscalerUpscaleRequestBodyArgs::default()
        .image("./image-data/Rabindranath_with_Einstein.jpeg")
        .build()?;

    let artifacts = client
        .generate("stable-diffusion-x4-latent-upscaler")
        .image_to_image_upscale(request)
        .await?;

    let paths = artifacts.save("./data/latent-upscaler").await?;

    paths
        .iter()
        .for_each(|path| println!("Image saved at {}", path.display()));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    latent_upscaler_upscale(&client).await?;

    real_esrgan_upscale(&client).await?;

    Ok(())
}
