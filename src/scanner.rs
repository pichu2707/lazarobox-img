use anyhow::{bail, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_images(input: &Path) -> Result<Vec<PathBuf>> {
    if !input.exists() {
        bail!("La ruta no existe: {:?}", input);
    }

    if input.is_file() {
        if is_image(input) {
            return Ok(vec![input.to_path_buf()]);
        }

        bail!("El archivo no parece una imagen válida: {:?}", input);
    }

    if input.is_dir() {
        let images = WalkDir::new(input)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path().to_path_buf())
            .filter(|path| is_image(path))
            .collect();

        return Ok(images);
    }

    bail!("La ruta no es un archivo ni una carpeta: {:?}", input);
}

fn is_image(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => {
            let ext = ext.to_lowercase();

            matches!(
                ext.as_str(),
                "jpg" | "jpeg" | "png" | "webp" | "bmp" | "tiff"
            )
        }
        None => false,
    }
}
