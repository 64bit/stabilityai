use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{download::save_b64, error::StabilityAIError, util::create_file_part};

use super::{
    Artifacts, ClipGuidancePreset, Image, ImageToImageRequestBody, ImageToImageUpscaleBody,
    InitImage, InputImage, LatentUpscalerUpscaleRequestBody, MaskImage, MaskSource,
    MaskingRequestBody, RealESRGANUpscaleRequestBody, Sampler, StylePreset,
};

use super::{TextPrompt, TextPrompts};

macro_rules! impl_from_for_text_prompt {
    ($from_typ:ty) => {
        impl From<$from_typ> for TextPrompt {
            fn from(value: $from_typ) -> Self {
                TextPrompt {
                    text: value.into(),
                    weight: None,
                }
            }
        }

        impl From<($from_typ, f64)> for TextPrompt {
            fn from(value: ($from_typ, f64)) -> Self {
                TextPrompt {
                    text: value.0.into(),
                    weight: Some(value.1.into()),
                }
            }
        }

        impl From<&($from_typ, f64)> for TextPrompt {
            fn from(value: &($from_typ, f64)) -> Self {
                TextPrompt {
                    text: value.0.clone().into(),
                    weight: Some(value.1.into()),
                }
            }
        }
    };
}

impl_from_for_text_prompt!(&str);
impl_from_for_text_prompt!(String);
impl_from_for_text_prompt!(&String);

macro_rules! impl_from_for_text_prompts {
    ($from_typ:ty) => {
        impl From<$from_typ> for TextPrompts {
            fn from(value: $from_typ) -> Self {
                TextPrompts {
                    text_prompts: vec![value.into()],
                }
            }
        }

        impl From<Vec<$from_typ>> for TextPrompts {
            fn from(value: Vec<$from_typ>) -> Self {
                TextPrompts {
                    text_prompts: value.into_iter().map(|v| v.into()).collect(),
                }
            }
        }

        impl From<&Vec<$from_typ>> for TextPrompts {
            fn from(value: &Vec<$from_typ>) -> Self {
                TextPrompts {
                    text_prompts: value.iter().map(|v| v.clone().into()).collect(),
                }
            }
        }

        impl<const N: usize> From<[$from_typ; N]> for TextPrompts {
            fn from(value: [$from_typ; N]) -> Self {
                TextPrompts {
                    text_prompts: value.iter().map(|v| v.clone().into()).collect(),
                }
            }
        }

        impl<const N: usize> From<&[$from_typ; N]> for TextPrompts {
            fn from(value: &[$from_typ; N]) -> Self {
                TextPrompts {
                    text_prompts: value.iter().map(|v| v.clone().into()).collect(),
                }
            }
        }
    };
}

impl_from_for_text_prompts!(&str);
impl_from_for_text_prompts!(String);
impl_from_for_text_prompts!(&String);
impl_from_for_text_prompts!((&str, f64));
impl_from_for_text_prompts!((String, f64));
impl_from_for_text_prompts!(&(String, f64));
impl_from_for_text_prompts!((&String, f64));

macro_rules! file_path_input {
    ($for_typ:ty) => {
        impl $for_typ {
            pub fn new<P: AsRef<Path>>(path: P) -> Self {
                Self {
                    path: PathBuf::from(path.as_ref()),
                }
            }
        }

        impl<P: AsRef<Path>> From<P> for $for_typ {
            fn from(path: P) -> Self {
                Self {
                    path: PathBuf::from(path.as_ref()),
                }
            }
        }
    };
}

file_path_input!(InitImage);
file_path_input!(InputImage);
file_path_input!(MaskImage);

impl Display for ClipGuidancePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FastBlue => "FAST_BLUE",
                Self::FastGreen => "FAST_GREEN",
                Self::None => "NONE",
                Self::Simple => "SIMPLE",
                Self::Slow => "SLOW",
                Self::Slower => "SLOWER",
                Self::Slowest => "SLOWEST",
            }
        )
    }
}

impl Display for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ddim => "DDIM",
                Self::Ddpm => "DDPM",
                Self::KDpmpp2m => "K_DPMPP_2M",
                Self::KDpmpp2sAncestral => "K_DPMPP_2S_ANCESTRAL",
                Self::KDpm2 => "K_DPM_2",
                Self::KDpm2Ancestral => "K_DPM_2_ANCESTRAL",
                Self::KEuler => "K_EULER",
                Self::KEulerAncestral => "K_EULER_ANCESTRAL",
                Self::KHeun => "K_HEUN",
                Self::KLms => "K_LMS",
            }
        )
    }
}

impl Display for StylePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ThreeDModel => "3d-model",
                Self::AnalogFilm => "analog-film",
                Self::Anime => "anime",
                Self::Cinematic => "cinematic",
                Self::ComicBook => "comic-book",
                Self::DigitalArt => "digital-art",
                Self::Enhance => "enhance",
                Self::FantasyArt => "fantasy-art",
                Self::Isometric => "isometric",
                Self::LineArt => "line-art",
                Self::LowPoly => "low-poly",
                Self::ModelingCompound => "modeling-compound",
                Self::NeonPunk => "neon-punk",
                Self::Origami => "origami",
                Self::Photographic => "photographic",
                Self::PixelArt => "pixel-art",
                Self::TileTexture => "tile-texture",
            }
        )
    }
}

impl Display for MaskSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MaskImageBlack => "MASK_IMAGE_BLACK",
                Self::MaskImageWhite => "MASK_IMAGE_WHITE",
                Self::InitImageAlpha => "INIT_IMAGE_ALPHA",
            }
        )
    }
}

impl Image {
    pub async fn save<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf, StabilityAIError> {
        match self.finish_reason {
            super::FinishReason::ContentFiltered => Err(StabilityAIError::FileSaveError(
                "FinishReason::CONTENT_FILTERED: Your request activated the API's safety
                filters and could not be processed. Please modify the prompt and try again."
                    .into(),
            )),
            super::FinishReason::Error => Err(StabilityAIError::FileSaveError(
                "FinishReason::ERROR".into(),
            )),
            super::FinishReason::Success => save_b64(&self.base64, dir).await,
        }
    }
}

impl Artifacts {
    /// Save each image in a dedicated Tokio task and return paths to saved files.
    pub async fn save<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>, StabilityAIError> {
        let exists = match Path::try_exists(dir.as_ref()) {
            Ok(exists) => exists,
            Err(e) => return Err(StabilityAIError::FileSaveError(e.to_string())),
        };

        if !exists {
            std::fs::create_dir_all(dir.as_ref())
                .map_err(|e| StabilityAIError::FileSaveError(e.to_string()))?;
        }

        let mut handles = vec![];
        for image in &self.artifacts {
            let dir_buf = PathBuf::from(dir.as_ref());
            let to_save = image.clone();
            handles.push(tokio::spawn(async move { to_save.save(dir_buf).await }));
        }

        let results = futures::future::join_all(handles).await;
        let mut errors = vec![];
        let mut paths = vec![];

        for result in results {
            match result {
                Ok(inner) => match inner {
                    Ok(path) => paths.push(path),
                    Err(e) => errors.push(e),
                },
                Err(e) => errors.push(StabilityAIError::FileSaveError(e.to_string())),
            }
        }

        if errors.is_empty() {
            Ok(paths)
        } else {
            Err(StabilityAIError::FileSaveError(
                errors
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("; "),
            ))
        }
    }
}

impl Default for ImageToImageUpscaleBody {
    fn default() -> Self {
        ImageToImageUpscaleBody::LatentUpscalerUpscaleRequestBody(Default::default())
    }
}

impl From<LatentUpscalerUpscaleRequestBody> for ImageToImageUpscaleBody {
    fn from(value: LatentUpscalerUpscaleRequestBody) -> Self {
        Self::LatentUpscalerUpscaleRequestBody(value)
    }
}

impl From<RealESRGANUpscaleRequestBody> for ImageToImageUpscaleBody {
    fn from(value: RealESRGANUpscaleRequestBody) -> Self {
        Self::RealESRGANUpscaleRequestBody(value)
    }
}

// start: types to multipart from

fn from_for_text_prompts(
    mut form: reqwest::multipart::Form,
    text_prompts: TextPrompts,
) -> reqwest::multipart::Form {
    for (idx, text_prompt) in text_prompts.text_prompts.into_iter().enumerate() {
        if text_prompt.text.is_empty() {
            continue;
        }
        form = form.text(format!("text_prompts[{idx}][text]"), text_prompt.text);
        if let Some(weight) = text_prompt.weight {
            form = form.text(format!("text_prompts[{idx}][weight]"), weight.to_string());
        }
    }
    form
}

#[async_convert::async_trait]
impl async_convert::TryFrom<ImageToImageRequestBody> for reqwest::multipart::Form {
    type Error = StabilityAIError;

    async fn try_from(request: ImageToImageRequestBody) -> Result<Self, Self::Error> {
        let init_image_part = create_file_part(&request.init_image.path).await?;

        let mut form = reqwest::multipart::Form::new().part("init_image", init_image_part);

        form = from_for_text_prompts(form, request.text_prompts);

        if let Some(image_strength) = request.image_strength {
            form = form.text("image_strength", image_strength.to_string());
        }

        if let Some(step_schedule_start) = request.step_schedule_start {
            form = form.text("step_schedule_start", step_schedule_start.to_string());
        }

        if let Some(step_schedule_end) = request.step_schedule_end {
            form = form.text("step_schedule_end", step_schedule_end.to_string());
        }

        if let Some(cfg_scale) = request.cfg_scale {
            form = form.text("cfg_scale", cfg_scale.to_string());
        }

        if let Some(clip_guidance_preset) = request.clip_guidance_preset {
            form = form.text("clip_guidance_preset", clip_guidance_preset.to_string());
        }

        if let Some(sampler) = request.sampler {
            form = form.text("sampler", sampler.to_string());
        }

        if let Some(samples) = request.samples {
            form = form.text("samples", samples.to_string());
        }

        if let Some(seed) = request.seed {
            form = form.text("seed", seed.to_string());
        }

        if let Some(steps) = request.steps {
            form = form.text("steps", steps.to_string());
        }

        if let Some(style_preset) = request.style_preset {
            form = form.text("style_preset", style_preset.to_string());
        }

        if let Some(extras) = request.extras {
            form = form.text("extras", extras.to_string());
        }

        Ok(form)
    }
}

#[async_convert::async_trait]
impl async_convert::TryFrom<LatentUpscalerUpscaleRequestBody> for reqwest::multipart::Form {
    type Error = StabilityAIError;

    async fn try_from(request: LatentUpscalerUpscaleRequestBody) -> Result<Self, Self::Error> {
        let image = create_file_part(&request.image.path).await?;

        let mut form = reqwest::multipart::Form::new().part("image", image);

        if let Some(width) = request.width {
            form = form.text("width", width.to_string());
        }

        if let Some(height) = request.height {
            form = form.text("height", height.to_string());
        }

        if let Some(text_prompts) = request.text_prompts {
            form = from_for_text_prompts(form, text_prompts);
        }

        if let Some(seed) = request.seed {
            form = form.text("seed", seed.to_string());
        }

        if let Some(steps) = request.steps {
            form = form.text("steps", steps.to_string());
        }

        if let Some(cfg_scale) = request.cfg_scale {
            form = form.text("cfg_scale", cfg_scale.to_string());
        }

        Ok(form)
    }
}

#[async_convert::async_trait]
impl async_convert::TryFrom<RealESRGANUpscaleRequestBody> for reqwest::multipart::Form {
    type Error = StabilityAIError;

    async fn try_from(request: RealESRGANUpscaleRequestBody) -> Result<Self, Self::Error> {
        let image = create_file_part(&request.image.path).await?;

        let mut form = reqwest::multipart::Form::new().part("image", image);

        if let Some(width) = request.width {
            form = form.text("width", width.to_string());
        }

        if let Some(height) = request.height {
            form = form.text("height", height.to_string());
        }

        Ok(form)
    }
}

#[async_convert::async_trait]
impl async_convert::TryFrom<ImageToImageUpscaleBody> for reqwest::multipart::Form {
    type Error = StabilityAIError;

    async fn try_from(request: ImageToImageUpscaleBody) -> Result<Self, Self::Error> {
        match request {
            ImageToImageUpscaleBody::LatentUpscalerUpscaleRequestBody(body) => {
                async_convert::TryFrom::try_from(body).await
            }
            ImageToImageUpscaleBody::RealESRGANUpscaleRequestBody(body) => {
                async_convert::TryFrom::try_from(body).await
            }
        }
    }
}

#[async_convert::async_trait]
impl async_convert::TryFrom<MaskingRequestBody> for reqwest::multipart::Form {
    type Error = StabilityAIError;

    async fn try_from(request: MaskingRequestBody) -> Result<Self, Self::Error> {
        let init_image = create_file_part(&request.init_image.path).await?;

        let mut form = reqwest::multipart::Form::new().part("init_image", init_image);

        form = from_for_text_prompts(form, request.text_prompts);

        form = form.text("mask_source", request.mask_source.to_string());

        if let Some(mask_image) = request.mask_image {
            let mask_image_part = create_file_part(mask_image.path).await?;
            form = form.part("mask_image", mask_image_part);
        }

        if let Some(cfg_scale) = request.cfg_scale {
            form = form.text("cfg_scale", cfg_scale.to_string());
        }

        if let Some(clip_guidance_preset) = request.clip_guidance_preset {
            form = form.text("clip_guidance_preset", clip_guidance_preset.to_string());
        }

        if let Some(sampler) = request.sampler {
            form = form.text("sampler", sampler.to_string());
        }

        if let Some(samples) = request.samples {
            form = form.text("samples", samples.to_string());
        }

        if let Some(seed) = request.seed {
            form = form.text("seed", seed.to_string());
        }

        if let Some(steps) = request.steps {
            form = form.text("steps", steps.to_string());
        }

        if let Some(style_preset) = request.style_preset {
            form = form.text("style_preset", style_preset.to_string());
        }

        if let Some(extras) = request.extras {
            form = form.text("extras", extras.to_string());
        }

        Ok(form)
    }
}

// end: types to multipart form
