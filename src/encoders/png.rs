use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use std::{fs, path::Path};

/// Codifica `image` como PNG (sin pérdida, por lo que `quality` no aplica).
pub fn save(image: &DynamicImage, output_path: &Path) -> Result<u64> {
    image.save_with_format(output_path, ImageFormat::Png)?;

    Ok(fs::metadata(output_path)?.len())
}
