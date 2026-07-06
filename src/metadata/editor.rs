//! Escritura de metadatos a disco (EXIF) mediante `little_exif`.
//!
//! Formatos validados de extremo a extremo (escribe + relee con kamadak-exif):
//! JPEG y WebP. Otros formatos devuelven un error claro.
//!
//! Cubre las acciones prioritarias: poner GPS, quitar GPS, quitar metadatos de
//! IA y establecer o borrar el alt text (descripción).

use anyhow::{Result, bail};
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use little_exif::rational::uR64;
use std::path::Path;

/// Cambios a aplicar sobre los metadatos de una imagen.
#[derive(Debug, Default, Clone)]
pub struct MetadataEdit {
    /// Alt text / descripción a establecer. `Some("")` lo borra; `None` no toca.
    pub alt_text: Option<String>,
    /// GPS a establecer (latitud, longitud en grados decimales).
    pub gps: Option<(f64, f64)>,
    /// Quitar las coordenadas GPS existentes.
    pub remove_gps: bool,
    /// Quitar metadatos relacionados con IA.
    pub remove_ai: bool,
}

/// Aplica `edit` sobre la imagen en `path`, escribiendo el resultado a disco.
pub fn apply(path: &Path, edit: &MetadataEdit) -> Result<()> {
    ensure_supported(path)?;

    // Se parte de los metadatos existentes para conservar los que no se tocan.
    let mut meta = Metadata::new_from_path(path).unwrap_or_else(|_| Metadata::new());

    if let Some(text) = &edit.alt_text {
        if text.trim().is_empty() {
            meta.remove_tag(ExifTag::ImageDescription(String::new()));
        } else {
            meta.set_tag(ExifTag::ImageDescription(text.clone()));
        }
    }

    if edit.remove_gps {
        remove_gps(&mut meta);
    } else if let Some((lat, lon)) = edit.gps {
        set_gps(&mut meta, lat, lon);
    }

    if edit.remove_ai {
        // El tag Software suele identificar la herramienta de IA generadora.
        meta.remove_tag(ExifTag::Software(String::new()));
    }

    meta.write_to_file(path)?;
    Ok(())
}

/// Rechaza formatos cuya escritura de metadatos no está validada.
fn ensure_supported(path: &Path) -> Result<()> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match ext.as_deref() {
        Some("jpg") | Some("jpeg") | Some("webp") => Ok(()),
        Some(other) => bail!("Escritura de metadatos no soportada en .{other} (usá JPEG o WebP)"),
        None => bail!("No se pudo determinar el formato de la imagen"),
    }
}

/// Establece latitud y longitud (grados decimales) como tags EXIF GPS.
fn set_gps(meta: &mut Metadata, lat: f64, lon: f64) {
    let lat_ref = if lat >= 0.0 { "N" } else { "S" };
    let lon_ref = if lon >= 0.0 { "E" } else { "W" };

    meta.set_tag(ExifTag::GPSLatitudeRef(lat_ref.to_string()));
    meta.set_tag(ExifTag::GPSLatitude(to_dms(lat)));
    meta.set_tag(ExifTag::GPSLongitudeRef(lon_ref.to_string()));
    meta.set_tag(ExifTag::GPSLongitude(to_dms(lon)));
}

/// Elimina los tags GPS de latitud y longitud.
fn remove_gps(meta: &mut Metadata) {
    meta.remove_tag(ExifTag::GPSLatitude(Vec::new()));
    meta.remove_tag(ExifTag::GPSLatitudeRef(String::new()));
    meta.remove_tag(ExifTag::GPSLongitude(Vec::new()));
    meta.remove_tag(ExifTag::GPSLongitudeRef(String::new()));
}

/// Convierte grados decimales a `[grados, minutos, segundos]` como racionales.
fn to_dms(decimal: f64) -> Vec<uR64> {
    let value = decimal.abs();
    let degrees = value.floor();
    let minutes_full = (value - degrees) * 60.0;
    let minutes = minutes_full.floor();
    let seconds = (minutes_full - minutes) * 60.0;

    vec![
        uR64 {
            nominator: degrees as u32,
            denominator: 1,
        },
        uR64 {
            nominator: minutes as u32,
            denominator: 1,
        },
        uR64 {
            nominator: (seconds * 1000.0).round() as u32,
            denominator: 1000,
        },
    ]
}
