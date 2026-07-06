mod jpeg;
mod png;
mod webp;

use anyhow::Result;
use image::DynamicImage;
use std::path::Path;

use crate::types::OutputFormat;

pub fn save_image(
    image: &DynamicImage,
    output_path: &Path,
    quality: u8,
    format: OutputFormat,
) -> Result<u64> {
    match format {
        OutputFormat::Webp => webp::save(image, output_path, quality),
        OutputFormat::Png => png::save(image, output_path),
        OutputFormat::Jpeg => jpeg::save(image, output_path, quality),
        OutputFormat::Avif => anyhow::bail!("El codificador AVIF aún no está implementado"),
    }
}
