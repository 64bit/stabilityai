use std::error::Error;

use stabilityai::{
    error::StabilityAIError,
    types::{ClipGuidancePreset, Sampler, StylePreset, TextPrompts, TextToImageRequestBodyArgs},
    Client,
};

async fn generate<T>(client: &Client, prompt: T) -> Result<(), StabilityAIError>
where
    T: Into<TextPrompts>,
{
    println!("Sending request, please be patient ...");
    let request = TextToImageRequestBodyArgs::default()
        .text_prompts(prompt)
        .samples(1)
        .steps(30_u32)
        .clip_guidance_preset(ClipGuidancePreset::FastBlue)
        .sampler(Sampler::KDpmpp2sAncestral)
        .width(1216_u16)
        .height(832_u16)
        .style_preset(StylePreset::ThreeDModel)
        .build()?;

    let artifacts = client
        .generate("stable-diffusion-xl-1024-v1-0")
        .text_to_image(request)
        .await?;

    let paths = artifacts.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image saved at {}", path.display()));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // generate using single prompt
    generate(
        &client,
        "A banner with ocean background having a cute \
        red cartoon crab on beach and boats in the ocean",
    )
    .await?;

    // generate using multiple prompts
    // generate(&client, ["A lighthouse by the ocean", "A boat"]).await?;

    // generate using prompts with weights
    // generate(&client, [("A lighthouse by the ocean", 0.4), ("Lots of boats", 0.6)]).await?;

    Ok(())
}
