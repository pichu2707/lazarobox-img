//! Módulo responsable de exportar imágenes optimizadas.
//!
//! Responsabilidades:
//! - Crear el directorio de salida.
//! - Construir las rutas de archivo finales.
//! - Más adelante. guardar imágenes en el disco.
//!
//! Este módulo no debe redimensionar ni optimizar imágenes.

use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Crea el directorio de salida para una imagen o carpeta
///
/// Ejemplo:
///
/// - `foto.jpg`->`optimized-foto`
/// - `imagenes/`-> `optimized-imagenes/`
///
/// Devuelve la ruta del directorio creado.
pub fn create_output_dir(input: &Path) -> Result<PathBuf> {
    let name = input
        .file_stem()
        .or_else(|| input.file_name())
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("No se pudo obtener el nombre de la entrada"))?;

    let base_dir = if input.is_dir() {
        input
    } else {
        input.parent().unwrap_or_else(|| Path::new("."))
    };

    let output_dir = base_dir.join(format!("optimized-{}", name));
    fs::create_dir_all(&output_dir)?;
    Ok(output_dir)
}

/// Construye la ruta final de una imagen optimziada.
///
/// Ejemplo:
/// original: `/imagenes/foto.jpg`
/// output_dir: `/imagenes/optimized-imagenes`
/// format: `webp`
///
/// Devuelve:
/// `/imagenes/optimized-imagenes/foto.webp`
pub fn create_output_file(
    output_dir: &Path,
    original_path: &Path,
    format: &str,
) -> Result<PathBuf> {
    let file_stem = original_path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("No se puede obetener el nombre del archivo"))?;

    let output_filename = format!("{}.{}", file_stem, format);
    Ok(output_dir.join(output_filename))
}
