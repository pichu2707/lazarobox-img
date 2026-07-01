//! Responsabilidad:
//! ----------------
//!
//! Módulo responsable de la optimización de imágenes.
//!
//! Este archivo contiene el pipeline interno de procesamiento:
//!
//! 1. Carga una imagen del disco.
//! 2. Redimensionarla en la memoria.
//! 3. Guardar versiones optimizadas.
//! 4. Devolver un resultado comparable.
//!
//! Este módulo no debe imprimir en terminal.
//! La salida visual pertenece a `report.rs`.
use crate::encoders;
use crate::inspector::{self, ImageInfo};
use crate::types::OutputFormat;
use anyhow::Result;
use image::DynamicImage;
use std::path::{Path, PathBuf};

/// Imagen ya cargada en memoria.
///
/// Combina:
/// - información ligera de la imagen
/// - píxeles reales cargados en RAM
#[derive(Debug)]
pub struct LoadedImage {
    pub info: ImageInfo,
    pub image: DynamicImage,
}

/// Representa una imagen generada por el optimziador
///
/// POr ejemplo:
/// - una versión en webp
/// - una versión en AVIF
/// - una versión en JPG comprimida
#[derive(Debug)]
pub struct OptimizedImage {
    /// Ruta donde se ha guardado la imagen optimizada.
    pub output_path: PathBuf,
    /// Formato generado: webp, avif, png, jpeg...
    pub format: OutputFormat,
    /// Tamañlo final e bytes
    pub size: u64,
    /// Anchura final en pixeles
    pub width: u32,
    /// Alutra final en píxeles
    pub height: u32,
}

/// Resultado completo de optimziar una imagen:
///
/// Contiene:
/// - La imagen original
/// - Una o varias versiones de la imagen optimizada
#[derive(Debug)]
pub struct OptimizationResult {
    pub original: ImageInfo,
    pub outputs: Vec<OptimizedImage>,
}

/// Optimiza una imagen
///
/// De momento no modifica ni guarda nada.
/// Solo crea un resultado falso apra validar la estructura.
pub fn optimize(
    path: &Path,
    output_path: &Path,
    max_width: Option<u32>,
    max_height: Option<u32>,
    quality: u8,
    format: OutputFormat,
) -> Result<OptimizationResult> {
    let loaded = load(path)?;

    let resized = resize_image(&loaded, max_width, max_height);

    let final_size = encoders::save_image(&resized, output_path, quality, format)?;

    let optimized_info = inspector::inspect(output_path)?;

    let optimized = OptimizedImage {
        output_path: output_path.to_path_buf(),
        format: format,
        size: final_size,
        width: optimized_info.width,
        height: optimized_info.height,
    };

    Ok(OptimizationResult {
        original: loaded.info,
        outputs: vec![optimized],
    })
}

/// Carga una imagen completa en memoria
///
/// Primero obtiene la información básica con `inspect`.
/// Depues abre la imagen real para poder modificar sus píxeles.
pub fn load(path: &Path) -> Result<LoadedImage> {
    let info = inspector::inspect(path)?;
    let image = image::open(path)?;

    Ok(LoadedImage { info, image })
}

/// Intenta redimensionar una imagen cargada.
///
/// Esta versión está hecha a propósito para que veamos
/// como Rust protege los datos prestados.
pub fn resize_image(
    loaded: &LoadedImage,
    max_width: Option<u32>,
    max_height: Option<u32>,
) -> DynamicImage {
    let (target_width, target_height) =
        calculate_target_size(loaded.info.width, loaded.info.height, max_width, max_height);

    loaded.image.resize(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
    )
}

///Calcular las dimensiones finales respetando la proporción original
///
///Casos:
///- si solo hay ancho, calcula el alto.
///- si solo hay alto, calcula el ancho
///- si hay un ancho y un alto, usa ambos como límites máximos.
///- si no hay ninguno, mantiene el tamaño original

pub fn calculate_target_size(
    original_width: u32,
    original_height: u32,
    max_width: Option<u32>,
    max_height: Option<u32>,
) -> (u32, u32) {
    match (max_width, max_height) {
        (Some(width), None) => {
            let ratio = width as f64 / original_width as f64;
            let height = (original_height as f64 * ratio).round() as u32;
            (width, height)
        }

        (None, Some(height)) => {
            let ratio = height as f64 / original_height as f64;
            let width = (original_width as f64 * ratio).round() as u32;
            (width, height)
        }

        (Some(width), Some(height)) => {
            let width_ratio = width as f64 / original_width as f64;
            let height_ratio = height as f64 / original_height as f64;
            let ratio = width_ratio.min(height_ratio);

            let final_width = (original_width as f64 * ratio).round() as u32;
            let final_height = (original_height as f64 * ratio).round() as u32;

            (final_width, final_height)
        }
        (None, None) => (original_width, original_height),
    }
}
