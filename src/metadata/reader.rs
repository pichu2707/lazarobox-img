use anyhow::Result;
use std::path::Path;

use crate::metadata::ImageMetadata;

///Lee los metadatos de una imagen.
///
///De momento solamente devuelve la estructura vacía
///Más adelante leerá EXIF, IPTC, ICC y datos relacionados con IA.

pub fn read_metadata(path: &Path) -> Result<ImageMetadata> {
    Ok(ImageMetadata::default())
}
