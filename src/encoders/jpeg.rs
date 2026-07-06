use anyhow::Result;
use image::DynamicImage;
use image::codecs::jpeg::JpegEncoder;
use std::{fs, path::Path};

/// Codifica `image` como JPEG respetando `quality` (1..=100).
pub fn save(image: &DynamicImage, output_path: &Path, quality: u8) -> Result<u64> {
    let mut bytes: Vec<u8> = Vec::new();

    let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
    image.write_with_encoder(encoder)?;

    fs::write(output_path, &bytes)?;

    Ok(bytes.len() as u64)
}
