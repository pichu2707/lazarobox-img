//! Escritura de metadatos a disco (EXIF) mediante `little_exif`.
//!
//! Formatos validados de extremo a extremo (escribe + relee con kamadak-exif):
//! JPEG y WebP. Otros formatos devuelven un error claro.
//!
//! Cubre las acciones prioritarias: poner GPS, quitar GPS, quitar metadatos de
//! IA y establecer o borrar el alt text (descripción).

use anyhow::{Context, Result, bail};
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use little_exif::rational::uR64;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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
        ensure_valid_coordinates(lat, lon)?;
        set_gps(&mut meta, lat, lon);
    }

    if edit.remove_ai {
        // El tag Software suele identificar la herramienta de IA generadora.
        meta.remove_tag(ExifTag::Software(String::new()));
    }

    write_with_backup(&meta, path)
}

fn write_with_backup(meta: &Metadata, path: &Path) -> Result<()> {
    let backup_path = create_backup(path)?;

    match meta.write_to_file(path) {
        Ok(()) => {
            // Si la escritura ya terminó correctamente, un fallo limpiando la
            // copia de seguridad no debe convertir el guardado en error: el
            // archivo del usuario ya contiene los metadatos nuevos.
            let _ = remove_backup(&backup_path);
            Ok(())
        }
        Err(write_error) => {
            if let Err(restore_error) = fs::copy(&backup_path, path) {
                bail!(
                    "Falló la escritura de metadatos ({write_error}) y no se pudo restaurar el original desde {}: {restore_error}",
                    backup_path.display()
                );
            }

            if let Err(cleanup_error) = remove_backup(&backup_path) {
                bail!(
                    "Falló la escritura de metadatos ({write_error}); el original fue restaurado desde {}, pero no se pudo eliminar la copia de seguridad: {cleanup_error}",
                    backup_path.display()
                );
            }

            bail!(
                "Falló la escritura de metadatos ({write_error}); el original fue restaurado desde {}",
                backup_path.display()
            )
        }
    }
}

fn create_backup(path: &Path) -> Result<PathBuf> {
    let backup_path = unique_backup_path(path)?;

    fs::copy(path, &backup_path).with_context(|| {
        format!(
            "No se pudo crear una copia de seguridad en {}",
            backup_path.display()
        )
    })?;

    Ok(backup_path)
}

fn remove_backup(path: &Path) -> Result<()> {
    fs::remove_file(path).with_context(|| {
        format!(
            "No se pudo eliminar la copia de seguridad {}",
            path.display()
        )
    })
}

fn unique_backup_path(path: &Path) -> Result<PathBuf> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .context("No se pudo determinar el nombre del archivo original")?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("La hora del sistema es anterior a UNIX_EPOCH")?
        .as_nanos();

    for attempt in 0..1000 {
        let mut backup_name = OsString::from(".");
        backup_name.push(file_name);
        backup_name.push(format!(
            ".lazarobox-backup-{}-{nanos}-{attempt}",
            std::process::id()
        ));
        let candidate = parent.join(backup_name);

        if !candidate.exists() {
            return Ok(candidate);
        }
    }

    bail!("No se pudo generar un nombre único para la copia de seguridad")
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

/// Rechaza coordenadas fuera del rango geográfico válido.
fn ensure_valid_coordinates(lat: f64, lon: f64) -> Result<()> {
    if !(-90.0..=90.0).contains(&lat) {
        bail!("Latitud GPS inválida: {lat} (debe estar entre -90 y 90)");
    }

    if !(-180.0..=180.0).contains(&lon) {
        bail!("Longitud GPS inválida: {lon} (debe estar entre -180 y 180)");
    }

    Ok(())
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

/// Elimina los tags EXIF GPS soportados por `little_exif`.
fn remove_gps(meta: &mut Metadata) {
    for tag in [
        ExifTag::GPSVersionID(Vec::new()),
        ExifTag::GPSLatitudeRef(String::new()),
        ExifTag::GPSLatitude(Vec::new()),
        ExifTag::GPSLongitudeRef(String::new()),
        ExifTag::GPSLongitude(Vec::new()),
        ExifTag::GPSAltitudeRef(Vec::new()),
        ExifTag::GPSAltitude(Vec::new()),
        ExifTag::GPSTimeStamp(Vec::new()),
        ExifTag::GPSSatellites(String::new()),
        ExifTag::GPSStatus(String::new()),
        ExifTag::GPSMeasureMode(String::new()),
        ExifTag::GPSDOP(Vec::new()),
        ExifTag::GPSSpeedRef(String::new()),
        ExifTag::GPSSpeed(Vec::new()),
        ExifTag::GPSTrackRef(String::new()),
        ExifTag::GPSTrack(Vec::new()),
        ExifTag::GPSImgDirectionRef(String::new()),
        ExifTag::GPSImgDirection(Vec::new()),
        ExifTag::GPSMapDatum(String::new()),
        ExifTag::GPSDestLatitudeRef(String::new()),
        ExifTag::GPSDestLatitude(Vec::new()),
        ExifTag::GPSDestLongitudeRef(String::new()),
        ExifTag::GPSDestLongitude(Vec::new()),
        ExifTag::GPSDestBearingRef(String::new()),
        ExifTag::GPSDestBearing(Vec::new()),
        ExifTag::GPSDestDistanceRef(String::new()),
        ExifTag::GPSDestDistance(Vec::new()),
        ExifTag::GPSProcessingMethod(Vec::new()),
        ExifTag::GPSAreaInformation(Vec::new()),
        ExifTag::GPSDateStamp(String::new()),
        ExifTag::GPSDifferential(Vec::new()),
        ExifTag::GPSHPositioningError(Vec::new()),
        ExifTag::GPSInfo(Vec::new()),
    ] {
        meta.remove_tag(tag);
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{MetadataStatus, read_metadata};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestImage {
        dir: PathBuf,
        path: PathBuf,
    }

    impl TestImage {
        fn jpeg(name: &str) -> Result<Self> {
            let dir = unique_temp_dir(name);
            fs::create_dir_all(&dir)?;

            let path = dir.join("image.jpg");
            image::RgbImage::from_pixel(1, 1, image::Rgb([255, 0, 0])).save(&path)?;

            Ok(Self { dir, path })
        }
    }

    impl Drop for TestImage {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "lazarobox-img-metadata-editor-{name}-{}-{nanos}",
            std::process::id()
        ))
    }

    fn assert_close(actual: f64, expected: f64) {
        let diff = (actual - expected).abs();
        assert!(
            diff < 0.000001,
            "expected {actual} to be within tolerance of {expected}, diff {diff}"
        );
    }

    #[test]
    fn creates_backup_next_to_original_with_same_content() -> Result<()> {
        let dir = unique_temp_dir("backup-content");
        fs::create_dir_all(&dir)?;
        let path = dir.join("image.jpg");
        fs::write(&path, b"original bytes")?;

        let backup_path = create_backup(&path)?;

        assert_eq!(backup_path.parent(), Some(dir.as_path()));
        assert!(
            backup_path
                .file_name()
                .and_then(|name| name.to_str())
                .expect("backup file name")
                .starts_with(".image.jpg.lazarobox-backup-")
        );
        assert_eq!(fs::read(&backup_path)?, b"original bytes");

        remove_backup(&backup_path)?;
        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn creates_unique_backup_names_for_same_original() -> Result<()> {
        let dir = unique_temp_dir("backup-unique");
        fs::create_dir_all(&dir)?;
        let path = dir.join("image.jpg");
        fs::write(&path, b"original bytes")?;

        let first_backup = create_backup(&path)?;
        let second_backup = create_backup(&path)?;

        assert_ne!(first_backup, second_backup);
        assert_eq!(fs::read(&first_backup)?, b"original bytes");
        assert_eq!(fs::read(&second_backup)?, b"original bytes");

        remove_backup(&first_backup)?;
        remove_backup(&second_backup)?;
        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    fn seed_software(path: &Path, software: &str) -> Result<()> {
        let mut meta = Metadata::new_from_path(path).unwrap_or_else(|_| Metadata::new());
        meta.set_tag(ExifTag::Software(software.to_string()));
        meta.write_to_file(path)?;
        Ok(())
    }

    fn seed_gps_altitude(path: &Path) -> Result<()> {
        let mut meta = Metadata::new_from_path(path).unwrap_or_else(|_| Metadata::new());
        meta.set_tag(ExifTag::GPSAltitudeRef(vec![0]));
        meta.set_tag(ExifTag::GPSAltitude(vec![uR64 {
            nominator: 650,
            denominator: 1,
        }]));
        meta.write_to_file(path)?;
        Ok(())
    }

    #[test]
    fn sets_ascii_alt_text_and_reader_can_reread_it() -> Result<()> {
        let image = TestImage::jpeg("set-alt")?;

        apply(
            &image.path,
            &MetadataEdit {
                alt_text: Some("Simple ASCII alt text".to_string()),
                ..Default::default()
            },
        )?;

        let metadata = read_metadata(&image.path)?;

        assert_eq!(
            metadata.web.description.value.as_deref(),
            Some("Simple ASCII alt text")
        );
        assert!(matches!(
            metadata.web.description.status,
            MetadataStatus::Present
        ));

        Ok(())
    }

    #[test]
    fn clears_alt_text() -> Result<()> {
        let image = TestImage::jpeg("clear-alt")?;

        apply(
            &image.path,
            &MetadataEdit {
                alt_text: Some("Text to clear".to_string()),
                ..Default::default()
            },
        )?;
        apply(
            &image.path,
            &MetadataEdit {
                alt_text: Some(String::new()),
                ..Default::default()
            },
        )?;

        let metadata = read_metadata(&image.path)?;

        assert_eq!(metadata.web.description.value, None);
        assert!(matches!(
            metadata.web.description.status,
            MetadataStatus::Missing
        ));

        Ok(())
    }

    #[test]
    fn sets_valid_gps_and_reader_can_reread_it() -> Result<()> {
        let image = TestImage::jpeg("set-gps")?;

        apply(
            &image.path,
            &MetadataEdit {
                gps: Some((40.416775, -3.703790)),
                ..Default::default()
            },
        )?;

        let metadata = read_metadata(&image.path)?;

        assert_close(metadata.seo.latitude.value.expect("latitude"), 40.416775);
        assert_close(metadata.seo.longitude.value.expect("longitude"), -3.703790);
        assert!(metadata.privacy.has_gps);

        Ok(())
    }

    #[test]
    fn changes_existing_gps_and_reader_sees_new_coordinates() -> Result<()> {
        let image = TestImage::jpeg("change-gps")?;

        apply(
            &image.path,
            &MetadataEdit {
                gps: Some((40.416775, -3.703790)),
                ..Default::default()
            },
        )?;
        apply(
            &image.path,
            &MetadataEdit {
                gps: Some((41.387397, 2.168568)),
                ..Default::default()
            },
        )?;

        let metadata = read_metadata(&image.path)?;

        let latitude = metadata.seo.latitude.value.expect("latitude");
        let longitude = metadata.seo.longitude.value.expect("longitude");
        assert_close(latitude, 41.387397);
        assert_close(longitude, 2.168568);
        assert!((latitude - 40.416775).abs() > 0.000001);
        assert!((longitude - -3.703790).abs() > 0.000001);
        assert!(metadata.privacy.has_gps);

        Ok(())
    }

    #[test]
    fn removes_gps_and_reader_reports_no_gps() -> Result<()> {
        let image = TestImage::jpeg("remove-gps")?;

        apply(
            &image.path,
            &MetadataEdit {
                gps: Some((40.416775, -3.703790)),
                ..Default::default()
            },
        )?;
        apply(
            &image.path,
            &MetadataEdit {
                remove_gps: true,
                ..Default::default()
            },
        )?;

        let metadata = read_metadata(&image.path)?;

        assert_eq!(metadata.seo.latitude.value, None);
        assert_eq!(metadata.seo.longitude.value, None);
        assert!(!metadata.privacy.has_gps);

        Ok(())
    }

    #[test]
    fn removes_extra_gps_tags_beyond_coordinates() -> Result<()> {
        let image = TestImage::jpeg("remove-extra-gps")?;

        apply(
            &image.path,
            &MetadataEdit {
                gps: Some((40.416775, -3.703790)),
                ..Default::default()
            },
        )?;
        seed_gps_altitude(&image.path)?;

        let before = Metadata::new_from_path(&image.path)?;
        assert_eq!(before.get_tag(&ExifTag::GPSAltitude(Vec::new())).count(), 1);

        apply(
            &image.path,
            &MetadataEdit {
                remove_gps: true,
                ..Default::default()
            },
        )?;

        let after = Metadata::new_from_path(&image.path)?;
        assert_eq!(after.get_tag(&ExifTag::GPSAltitude(Vec::new())).count(), 0);
        assert_eq!(
            after.get_tag(&ExifTag::GPSAltitudeRef(Vec::new())).count(),
            0
        );

        Ok(())
    }

    #[test]
    fn removes_software_ai_marker() -> Result<()> {
        let image = TestImage::jpeg("remove-ai")?;
        seed_software(&image.path, "Stable Diffusion")?;

        let before = read_metadata(&image.path)?;
        assert_eq!(
            before.ai.software.value.as_deref(),
            Some("Stable Diffusion")
        );
        assert!(before.ai.detected);

        apply(
            &image.path,
            &MetadataEdit {
                remove_ai: true,
                ..Default::default()
            },
        )?;

        let after = read_metadata(&image.path)?;
        assert_eq!(after.ai.software.value, None);
        assert!(!after.ai.detected);

        Ok(())
    }

    #[test]
    fn rejects_unsupported_extension_with_clear_error() {
        let path = Path::new("image.png");

        let error = apply(path, &MetadataEdit::default()).expect_err("png should be rejected");

        assert!(
            error
                .to_string()
                .contains("Escritura de metadatos no soportada en .png")
        );
    }

    #[test]
    fn rejects_latitude_outside_valid_range() -> Result<()> {
        let image = TestImage::jpeg("invalid-lat")?;

        let error = apply(
            &image.path,
            &MetadataEdit {
                gps: Some((91.0, 0.0)),
                ..Default::default()
            },
        )
        .expect_err("invalid latitude should be rejected");

        assert!(error.to_string().contains("Latitud GPS inválida"));

        Ok(())
    }

    #[test]
    fn rejects_longitude_outside_valid_range() -> Result<()> {
        let image = TestImage::jpeg("invalid-lon")?;

        let error = apply(
            &image.path,
            &MetadataEdit {
                gps: Some((0.0, 181.0)),
                ..Default::default()
            },
        )
        .expect_err("invalid longitude should be rejected");

        assert!(error.to_string().contains("Longitud GPS inválida"));

        Ok(())
    }
}
