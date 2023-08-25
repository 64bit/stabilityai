use std::fmt::Display;

use crate::{
    error::StabilityAIError,
    types::{
        Artifacts, ImageToImageRequestBody, ImageToImageUpscaleBody, MaskingRequestBody,
        TextToImageRequestBody,
    },
    Client,
};

/// Generate images from text, existing images, or both
pub struct Generate<'c, E: Display> {
    client: &'c Client,
    engine_id: E,
}

impl<'c, E: Display> Generate<'c, E> {
    pub fn new(client: &'c Client, engine_id: E) -> Self {
        Self { client, engine_id }
    }

    /// Generate a new image from a text prompt
    pub async fn text_to_image(
        &self,
        request: TextToImageRequestBody,
    ) -> Result<Artifacts, StabilityAIError> {
        self.client
            .post(
                &format!("/generation/{}/text-to-image", self.engine_id),
                request,
            )
            .await
    }

    /// Modify an image based on a text prompt
    pub async fn image_to_image(
        &self,
        request: ImageToImageRequestBody,
    ) -> Result<Artifacts, StabilityAIError> {
        self.client
            .post_form(
                &format!("/generation/{}/image-to-image", self.engine_id),
                request,
            )
            .await
    }

    /// Create a higher resolution version of an input image.
    ///
    ///
    /// This operation outputs an image with a maximum pixel count of **4,194,304**.
    /// This is equivalent to dimensions such as `2048x2048` and `4096x1024`.
    ///
    ///
    /// By default, the input image will be upscaled by a factor of 2.
    /// For additional control over the output dimensions, a `width` or `height`
    /// parameter may be specified.\n\nFor upscaler engines that are ESRGAN-based,
    /// refer to the `RealESRGANUpscaleRequestBody` body option below. For upscaler
    /// engines that are Stable Diffusion Latent Upscaler-based, refer to the
    /// `LatentUpscalerUpscaleRequestBody` body option below.
    ///
    ///
    /// For more details on the upscaler engines, refer to the
    /// [documentation on the Platform site.](https://platform.stability.ai/docs/features/image-upscaling?tab=python)
    pub async fn image_to_image_upscale<R: Into<ImageToImageUpscaleBody>>(
        &self,
        request: R,
    ) -> Result<Artifacts, StabilityAIError> {
        self.client
            .post_form(
                &format!("/generation/{}/image-to-image/upscale", self.engine_id),
                request.into(),
            )
            .await
    }

    /// Selectively modify portions of an image using a mask
    pub async fn image_to_image_masking(
        &self,
        request: MaskingRequestBody,
    ) -> Result<Artifacts, StabilityAIError> {
        self.client
            .post_form(
                &format!("/generation/{}/image-to-image/masking", self.engine_id),
                request,
            )
            .await
    }
}
