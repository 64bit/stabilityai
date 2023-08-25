<h1 align="center"> stabilityai </h1>
<p align="center">Rust library for stability.ai </p>
<div align="center">
    <a href="https://crates.io/crates/stabilityai">
    <img src="https://img.shields.io/crates/v/stabilityai.svg" />
    </a>
    <a href="https://docs.rs/stabilityai">
    <img src="https://docs.rs/stabilityai/badge.svg" />
    </a>
</div>

## Overview

`stabilityai` is an unofficial Rust library for stability.ai

- It's based on [OpenAPI spec](https://platform.stability.ai/docs/api-reference)
- Current features:
  - [x] Users
  - [x] Engines
  - [x] Generation

The library reads [API key](https://platform.stability.ai/account/keys) from the environment variable `STABILITY_API_KEY`.

```bash
# On macOS/Linux
export STABILITY_API_KEY='sk-...'
```

```powershell
# On Windows Powershell
$Env:STABILITY_API_KEY='sk-...'
```

- Visit [examples](https://github.com/64bit/stabilityai/tree/main/examples) for full examples.
- Visit [docs.rs/stabilityai](https://docs.rs/stabilityai) for docs.

## Text To Image Example

```rust
use stabilityai::{
    error::StabilityAIError,
    types::{ClipGuidancePreset, Sampler, StylePreset, TextToImageRequestBodyArgs},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads STABILITY_API_KEY environment variable for API key.
    let client = Client::new();

    let request = TextToImageRequestBodyArgs::default()
        .text_prompts(
            "A banner with ocean background having a cute \
            red cartoon crab on beach and boats in the ocean",
        )
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

    // Create directory if doesn't exist and save images
    let paths = artifacts.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image saved at {}", path.display()));

    Ok(())
}
```

<div align="center">
  <img width="600" src="https://raw.githubusercontent.com/64bit/stabilityai/assets/examples/text-to-image/data/7uaqAjP7uz.png" />
</div>

## Contributing

Thank you for your time to contribute and improve the project, I'd be happy to have you!

A good starting point cloud be an existing [open issue](https://github.com/64bit/stabilityai/issues).

## License

This project is licensed under [MIT license](https://github.com/64bit/stabilityai/blob/main/LICENSE).
