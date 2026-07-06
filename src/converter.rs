//! Conversión de formato de imágenes.
//!
//! Carga una imagen y la recodifica a un formato de destino, escribiendo el
//! resultado junto a la entrada dentro de una carpeta `optimized-<nombre>/`.
//! Reutiliza los módulos `export` (rutas) y `encoders` (codificación).
//!
//! Este módulo no redimensiona imágenes; eso pertenece a `optimizer`.

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::{encoders, export, types::OutputFormat};

/// Resultado de una conversión correcta.
#[derive(Debug)]
pub struct Conversion {
    pub output_path: PathBuf,
    pub size: u64,
}

/// Convierte `input` a `format` con la `quality` indicada.
pub fn convert(input: &Path, format: OutputFormat, quality: u8) -> Result<Conversion> {
    let image = image::open(input)?;

    let output_dir = export::create_output_dir(input)?;
    let output_path = export::create_output_file(&output_dir, input, format.extension())?;

    let size = encoders::save_image(&image, &output_path, quality, format)?;

    Ok(Conversion { output_path, size })
}
