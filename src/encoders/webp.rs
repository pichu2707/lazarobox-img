use anyhow::Result;
use image::DynamicImage;
use std::{fs, path::Path};

pub fn save(image: &DynamicImage, output_path: &Path, quality: u8) -> Result<u64> {
    let encoder =
        webp::Encoder::from_image(image).map_err(|err| anyhow::anyhow!(err.to_string()))?;

    let webp_data = encoder.encode(quality as f32);

    fs::write(output_path, &*webp_data)?;

    let metadata = fs::metadata(output_path)?;

    Ok(metadata.len())
}
