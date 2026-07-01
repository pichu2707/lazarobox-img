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
        OutputFormat::Avif => todo!("AVIF todavía no implementado"),
        OutputFormat::Jpeg => todo!("JPEG todavía no implementado"),
        OutputFormat::Png => todo!("PNG todavía no implementado"),
    }
}
