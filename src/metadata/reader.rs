use anyhow::Result;
use std::{fs::File, io::BufReader, path::Path};

use crate::metadata::{ImageMetadata, parsers};

///Lee los metadatos de una imagen.
///
///De momento solamente devuelve la estructura vacía
///Más adelante leerá EXIF, IPTC, ICC y datos relacionados con IA.

pub fn read_metadata(path: &Path) -> Result<ImageMetadata> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let exif = exif::Reader::new().read_from_container(&mut reader);

    match exif {
        Ok(exif_data) => {
            let mut metadata = ImageMetadata::default();
            parsers::parse_all(&exif_data, &mut metadata);
            Ok(metadata)
        }

        Err(_) => Ok(ImageMetadata::default()),
    }
}
