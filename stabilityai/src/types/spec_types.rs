use std::{path::PathBuf, sync::Arc};

use derive_builder::Builder;

use serde::{Deserialize, Serialize};

use crate::error::StabilityAIError;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OrganizationMembership {
    pub id: String,
    pub is_default: bool,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct AccountResponseBody {
    /// The user's email
    pub email: String,
    /// The user's ID
    pub id: String,
    /// The user's organizations
    pub organizations: Vec<OrganizationMembership>,
    /// The user's profile picture
    pub profile_picture: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BalanceResponseBody {
    /// The balance of the account/organization associated with the API key
    pub credits: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum EngineType {
    AUDIO,
    CLASSIFICATION,
    PICTURE,
    STORAGE,
    TEXT,
    VIDEO,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Engine {
    /// Unique identifier for the engine
    pub id: String,
    /// Name of the engine
    pub name: String,
    pub description: String,
    /// The type of content this engine produces
    pub r#type: EngineType,
}

/// Text prompt for image generation
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TextPrompt {
    /// The prompt itself
    pub text: String,
    /// Weight of the prompt (use negative numbers for negative prompts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ClipGuidancePreset {
    #[serde(rename = "FAST_BLUE")]
    FastBlue,
    #[serde(rename = "FAST_GREEN")]
    FastGreen,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "SLOW")]
    Slow,
    #[serde(rename = "SLOWER")]
    Slower,
    #[serde(rename = "SLOWEST")]
    Slowest,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Sampler {
    #[serde(rename = "DDIM")]
    Ddim,
    #[serde(rename = "DDPM")]
    Ddpm,
    #[serde(rename = "K_DPMPP_2M")]
    KDpmpp2m,
    #[serde(rename = "K_DPMPP_2S_ANCESTRAL")]
    KDpmpp2sAncestral,
    #[serde(rename = "K_DPM_2")]
    KDpm2,
    #[serde(rename = "K_DPM_2_ANCESTRAL")]
    KDpm2Ancestral,
    #[serde(rename = "K_EULER")]
    KEuler,
    #[serde(rename = "K_EULER_ANCESTRAL")]
    KEulerAncestral,
    #[serde(rename = "K_HEUN")]
    KHeun,
    #[serde(rename = "K_LMS")]
    KLms,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum StylePreset {
    #[serde(rename = "3d-model")]
    ThreeDModel,
    #[serde(rename = "analog-film")]
    AnalogFilm,
    #[serde(rename = "anime")]
    Anime,
    #[serde(rename = "cinematic")]
    Cinematic,
    #[serde(rename = "comic-book")]
    ComicBook,
    #[serde(rename = "digital-art")]
    DigitalArt,
    #[serde(rename = "enhance")]
    Enhance,
    #[serde(rename = "fantasy-art")]
    FantasyArt,
    #[serde(rename = "isometric")]
    Isometric,
    #[serde(rename = "line-art")]
    LineArt,
    #[serde(rename = "low-poly")]
    LowPoly,
    #[serde(rename = "modeling-compound")]
    ModelingCompound,
    #[serde(rename = "neon-punk")]
    NeonPunk,
    #[serde(rename = "origami")]
    Origami,
    #[serde(rename = "photographic")]
    Photographic,
    #[serde(rename = "pixel-art")]
    PixelArt,
    #[serde(rename = "tile-texture")]
    TileTexture,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct TextPrompts {
    pub text_prompts: Vec<TextPrompt>,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq, Builder)]
#[builder(name = "TextToImageRequestBodyArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "StabilityAIError"))]
pub struct TextToImageRequestBody {
    /// An array of text prompts to use for generation.
    ///
    ///
    /// Due to how arrays are represented in `multipart/form-data` requests,
    ///  prompts must adhere to the format `text_prompts[index][text|weight]`,
    ///
    /// where `index` is some integer used to tie the text and weight together.
    /// While `index` does not have to be sequential, duplicate entries
    /// will override previous entries, so it is recommended to use sequential
    /// indices.
    ///
    ///
    /// Given a text prompt with the text `A lighthouse on a cliff` and a weight
    /// of `0.5`, it would be represented as:
    ///
    /// `text_prompts[0][text]: "A lighthouse on a cliff"`
    ///
    /// `text_prompts[0][weight]: 0.5`
    ///
    ///
    /// To add another prompt to that request simply provide the values
    /// under a new `index`:
    ///
    ///
    /// `text_prompts[0][text]: "A lighthouse on a cliff"`
    ///
    /// `text_prompts[0][weight]: 0.5`
    ///
    /// `text_prompts[1][text]: "land, ground, dirt, grass"`
    ///
    /// `text_prompts[1][weight]: -0.9`
    #[serde(flatten)]
    pub text_prompts: TextPrompts,

    /// Height of the image in pixels.  Must be in increments of 64 and pass the
    /// following validation:
    ///
    /// - For 512 engines: 262,144 ≤ `height * width` ≤ 1,048,576
    ///
    /// - For 768 engines: 589,824 ≤ `height * width` ≤ 1,048,576
    ///
    /// - For SDXL Beta: can be as low as 128 and as high as 896 as long as `width`
    /// is not greater than 512. If `width` is greater than 512 then this can
    /// be _at most_ 512.
    ///
    /// - For SDXL v0.9: valid dimensions are 1024x1024, 1152x896, 1216x832,
    /// 1344x768, 1536x640, 640x1536, 768x1344, 832x1216, or 896x1152
    ///
    /// - For SDXL v1.0: valid dimensions are the same as SDXL v0.9
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,

    /// Width of the image in pixels.  Must be in increments of 64 and pass the
    /// following validation:
    ///
    /// - For 512 engines: 262,144 ≤ `height * width` ≤ 1,048,576
    ///
    /// - For 768 engines: 589,824 ≤ `height * width` ≤ 1,048,576
    ///
    /// - For SDXL Beta: can be as low as 128 and as high as 896 as long as `height`
    /// is not greater than 512. If `height` is greater than 512 then this can be _at most_ 512.
    ///
    /// - For SDXL v0.9: valid dimensions are 1024x1024, 1152x896, 1216x832, 1344x768, 1536x640,
    /// 640x1536, 768x1344, 832x1216, or 896x1152
    ///
    /// - For SDXL v1.0: valid dimensions are the same as SDXL v0.9
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,

    /// How strictly the diffusion process adheres to the prompt text
    /// (higher values keep your image closer to your prompt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg_scale: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_guidance_preset: Option<ClipGuidancePreset>,

    /// Which sampler to use for the diffusion process.
    /// If this value is omitted we'll automatically select
    /// an appropriate sampler for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<Sampler>,

    /// Number of images to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<u8>,

    /// Random noise seed (omit this option or use `0` for a random seed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Number of diffusion steps to run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<u32>,

    /// Pass in a style preset to guide the image model towards a particular style.
    ///
    /// This list of style presets is subject to change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_preset: Option<StylePreset>,

    /// Extra parameters passed to the engine.
    ///
    /// These parameters are used for in-development or experimental features
    /// and may change without warning, so please use with caution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum FinishReason {
    #[serde(rename = "CONTENT_FILTERED")]
    ContentFiltered,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "SUCCESS")]
    Success,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Artifacts {
    pub artifacts: Vec<Arc<Image>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Image {
    pub base64: String,
    #[serde(rename = "finishReason")]
    pub finish_reason: FinishReason,
    pub seed: i64,
}

#[derive(Debug, Deserialize, Default, Serialize, Clone, PartialEq)]
pub enum InitImageMode {
    #[default]
    #[serde(rename = "IMAGE_STRENGTH")]
    ImageStrength,
    #[serde(rename = "STEP_SCHEDULE")]
    StepSchedule,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InitImage {
    pub path: PathBuf,
}

#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(name = "ImageToImageRequestBodyArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "StabilityAIError"))]
pub struct ImageToImageRequestBody {
    pub text_prompts: TextPrompts,

    pub init_image: InitImage,

    /// Whether to use `image_strength` or `step_schedule_*` to control how
    /// much influence the `init_image` has on the result.
    pub init_image_mode: Option<InitImageMode>,

    /// How much influence the `init_image` has on the diffusion process.
    /// Values close to `1` will yield images very similar to the `init_image`
    ///  while values close to `0` will yield images wildly different than
    /// the `init_image`. The behavior of this is meant to mirror DreamStudio's
    ///  \"Image Strength\" slider.  <br/> <br/> This parameter is just an
    /// alternate way to set `step_schedule_start`, which is done via the
    /// calculation `1 - image_strength`. For example, passing in an Image
    /// Strength of 35% (`0.35`) would result in a `step_schedule_start` of
    /// `0.65`.\n"
    pub image_strength: Option<f64>,

    /// Skips a proportion of the start of the diffusion steps, allowing the
    /// init_image to influence the final generated image.  Lower values will
    ///  result in more influence from the init_image, while higher values will
    ///  result in more influence from the diffusion steps.  (e.g. a value
    /// of `0` would simply return you the init_image, where a value of `1`
    /// would return you a completely different image.)
    pub step_schedule_start: Option<f64>,

    /// Skips a proportion of the end of the diffusion steps, allowing the
    /// init_image to influence the final generated image.  Lower values will
    ///  result in more influence from the init_image, while higher values will
    /// result in more influence from the diffusion steps.
    pub step_schedule_end: Option<f64>,

    /// How strictly the diffusion process adheres to the prompt text
    /// (higher values keep your image closer to your prompt)
    pub cfg_scale: Option<u8>,

    pub clip_guidance_preset: Option<ClipGuidancePreset>,

    /// Which sampler to use for the diffusion process.
    /// If this value is omitted we'll automatically select
    /// an appropriate sampler for you.
    pub sampler: Option<Sampler>,

    /// Number of images to generate
    pub samples: Option<u8>,

    /// Random noise seed (omit this option or use `0` for a random seed)
    pub seed: Option<u32>,

    /// Number of diffusion steps to run
    pub steps: Option<u32>,

    /// Pass in a style preset to guide the image model towards a particular style.
    ///
    /// This list of style presets is subject to change.
    pub style_preset: Option<StylePreset>,

    /// Extra parameters passed to the engine.
    ///
    /// These parameters are used for in-development or experimental features
    /// and may change without warning, so please use with caution.
    pub extras: Option<serde_json::Value>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InputImage {
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageToImageUpscaleBody {
    LatentUpscalerUpscaleRequestBody(LatentUpscalerUpscaleRequestBody),
    RealESRGANUpscaleRequestBody(RealESRGANUpscaleRequestBody),
}

#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(name = "RealESRGANUpscaleRequestBodyArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "StabilityAIError"))]
pub struct RealESRGANUpscaleRequestBody {
    pub image: InputImage,

    /// Desired height of the output image.
    /// Only one of `width` or `height` may be specified.
    pub height: Option<u16>,

    /// Desired width of the output image.
    /// Only one of `width` or `height` may be specified.
    pub width: Option<u16>,
}

#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(name = "LatentUpscalerUpscaleRequestBodyArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "StabilityAIError"))]
pub struct LatentUpscalerUpscaleRequestBody {
    pub image: InputImage,

    pub text_prompts: Option<TextPrompts>,

    /// Desired height of the output image.
    /// Only one of `width` or `height` may be specified.
    pub height: Option<u16>,

    /// Desired width of the output image.
    /// Only one of `width` or `height` may be specified.
    pub width: Option<u16>,

    /// How strictly the diffusion process adheres to the prompt text
    /// (higher values keep your image closer to your prompt)
    pub cfg_scale: Option<u8>,

    /// Random noise seed (omit this option or use `0` for a random seed)
    pub seed: Option<u32>,

    /// Number of diffusion steps to run
    pub steps: Option<u32>,
}

#[derive(Debug, Deserialize, Default, Serialize, Clone, PartialEq)]
pub enum MaskSource {
    #[default]
    #[serde(rename = "MASK_IMAGE_BLACK")]
    MaskImageBlack,
    #[serde(rename = "MASK_IMAGE_WHITE")]
    MaskImageWhite,
    #[serde(rename = "INIT_IMAGE_ALPHA")]
    InitImageAlpha,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MaskImage {
    pub path: PathBuf,
}

#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(name = "MaskingRequestBodyArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "StabilityAIError"))]
pub struct MaskingRequestBody {
    /// Image used to initialize the diffusion process, in lieu of random noise.
    pub init_image: InitImage,

    /// For any given pixel, the mask determines the strength of generation
    /// on a linear scale.  This parameter determines where to source the
    /// mask from:
    ///
    /// - `MASK_IMAGE_WHITE` will use the white pixels of the
    /// mask_image as the mask, where white pixels are completely replaced
    /// and black pixels are unchanged
    ///
    /// - `MASK_IMAGE_BLACK` will use the
    /// black pixels of the mask_image as the mask, where black pixels are
    /// completely replaced and white pixels are unchanged
    ///
    /// - `INIT_IMAGE_ALPHA` will use the alpha channel of the init_image
    /// as the mask, where fully transparent pixels are completely replaced
    /// and fully opaque pixels are unchanged
    pub mask_source: MaskSource,

    /// Optional grayscale mask that allows for influence over which pixels
    /// are eligible for diffusion and at what strength. Must be the same
    /// dimensions as the `init_image`. Use the `mask_source` option to
    /// specify whether the white or black pixels should be inpainted.
    pub mask_image: Option<MaskImage>,

    pub text_prompts: TextPrompts,

    /// How strictly the diffusion process adheres to the prompt text
    /// (higher values keep your image closer to your prompt)
    pub cfg_scale: Option<u8>,

    pub clip_guidance_preset: Option<ClipGuidancePreset>,

    /// Which sampler to use for the diffusion process.
    /// If this value is omitted we'll automatically select
    /// an appropriate sampler for you.
    pub sampler: Option<Sampler>,

    /// Number of images to generate
    pub samples: Option<u8>,

    /// Random noise seed (omit this option or use `0` for a random seed)
    pub seed: Option<u32>,

    /// Number of diffusion steps to run
    pub steps: Option<u32>,

    /// Pass in a style preset to guide the image model towards a particular style.
    ///
    /// This list of style presets is subject to change.
    pub style_preset: Option<StylePreset>,

    /// Extra parameters passed to the engine.
    ///
    /// These parameters are used for in-development or experimental features
    /// and may change without warning, so please use with caution.
    pub extras: Option<serde_json::Value>,
}
