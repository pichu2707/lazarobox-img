use anyhow::Result;
use image::GenericImageView;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Contiene la información básica de una imagen.
///
/// En esta primera versión únicamente almacenamos
/// los datos originales.
/// Más adelante añadiremos también la información

#[derive(Debug)]
pub struct ImageInfo {
    pub path: PathBuf,
    pub filename: String,
    pub size: u64,
    pub width: u32,
    pub height: u32,
}

pub fn inspect(path: &Path) -> Result<ImageInfo> {
    // ------------------------------------------------
    // Paso 1.
    // Pedimos al sistema operativo la metadata
    // del archivo.
    // ------------------------------------------------
    let metadata = fs::metadata(path)?;

    // ------------------------------------------------
    // Paso 2.
    // Abrimos la imagen.
    // ------------------------------------------------
    // image::open detecta automáticamente
    // el formato.
    // ------------------------------------------------
    let image = image::open(path)?;

    // ------------------------------------------------
    // Paso 3.
    // Obtenemos la resolución.
    // ------------------------------------------------
    let (width, height) = image.dimensions();

    // ------------------------------------------------
    // Paso 4.
    // Obtenemos únicamente el nombre del archivo.
    // ------------------------------------------------
    let filename = path.file_name().unwrap().to_string_lossy().to_string();

    // ------------------------------------------------
    // Paso 5.
    // Construimos nuestra struct.
    // ------------------------------------------------
    Ok(ImageInfo {
        path: path.to_path_buf(),
        filename,
        size: metadata.len(),
        width,
        height,
    })
}
