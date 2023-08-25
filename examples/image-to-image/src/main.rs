use std::error::Error;

use stabilityai::{
    types::{ClipGuidancePreset, ImageToImageRequestBodyArgs, InitImageMode, Sampler},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = ImageToImageRequestBodyArgs::default()
        .text_prompts((
            "crayon drawing of a banner with ocean background having a cute red \
            cartoon crab on beach and boats in the ocean",
            1.0,
        ))
        .init_image("./image-data/crab-beach-boats.png")
        .init_image_mode(InitImageMode::ImageStrength)
        .seed(123463446_u32)
        .steps(20_u32)
        .cfg_scale(8)
        .samples(1)
        .clip_guidance_preset(ClipGuidancePreset::FastBlue)
        .sampler(Sampler::KDpm2Ancestral)
        .build()?;

    println!("Sending request, please be patient ...");
    let artifacts = client
        .generate("stable-diffusion-xl-1024-v1-0")
        .image_to_image(request)
        .await?;

    let paths = artifacts.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image saved at {}", path.display()));

    Ok(())
}
