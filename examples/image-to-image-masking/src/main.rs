use std::error::Error;

use stabilityai::{
    types::{ClipGuidancePreset, MaskSource, MaskingRequestBodyArgs, Sampler},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = MaskingRequestBodyArgs::default()
        .text_prompts([
            (
                "Lake side resort by mountains and snow, hyper realistic, sun behind the mountains",
                1.0,
            ),
            ("outdoors boats in the lake ", 0.5),
            ("inside. indoors", -100.0),
        ])
        .init_image("./image-data/Inpainting-C1.png")
        .mask_image("./image-data/Inpainting-C2.png")
        .mask_source(MaskSource::MaskImageWhite)
        .steps(20_u32)
        .cfg_scale(8)
        .samples(1)
        .clip_guidance_preset(ClipGuidancePreset::FastBlue)
        .sampler(Sampler::KDpm2Ancestral)
        .build()?;

    println!("Sending request, please be patient ...");
    let artifacts = client
        .generate("stable-diffusion-xl-1024-v1-0")
        .image_to_image_masking(request)
        .await?;

    let paths = artifacts.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image saved at {}", path.display()));

    Ok(())
}
