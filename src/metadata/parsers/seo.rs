use crate::metadata::{ImageMetadata, MetadataStatus};

/// Parsea las coordenadas GPS (si existen) a grados decimales y marca la
/// presencia de datos de ubicación.
pub fn parse(exif_data: &exif::Exif, metadata: &mut ImageMetadata) {
    let latitude = read_coordinate(
        exif_data,
        exif::Tag::GPSLatitude,
        exif::Tag::GPSLatitudeRef,
        'S',
    );

    let longitude = read_coordinate(
        exif_data,
        exif::Tag::GPSLongitude,
        exif::Tag::GPSLongitudeRef,
        'W',
    );

    if let Some(value) = latitude {
        metadata.seo.latitude.value = Some(value);
        metadata.seo.latitude.status = MetadataStatus::Present;
        metadata.privacy.has_gps = true;
    }

    if let Some(value) = longitude {
        metadata.seo.longitude.value = Some(value);
        metadata.seo.longitude.status = MetadataStatus::Present;
        metadata.privacy.has_gps = true;
    }
}

/// Lee una coordenada GPS (grados/minutos/segundos) y la devuelve en grados
/// decimales, aplicando el signo según el hemisferio (`negative_ref` = S o W).
fn read_coordinate(
    exif_data: &exif::Exif,
    coord_tag: exif::Tag,
    ref_tag: exif::Tag,
    negative_ref: char,
) -> Option<f64> {
    let field = exif_data.get_field(coord_tag, exif::In::PRIMARY)?;

    let dms = match &field.value {
        exif::Value::Rational(values) if values.len() >= 3 => values,
        _ => return None,
    };

    let decimal =
        dms[0].to_f64() + dms[1].to_f64() / 60.0 + dms[2].to_f64() / 3600.0;

    let sign = exif_data
        .get_field(ref_tag, exif::In::PRIMARY)
        .map(|field| field.display_value().to_string())
        .map(|reference| {
            if reference.trim().to_uppercase().starts_with(negative_ref) {
                -1.0
            } else {
                1.0
            }
        })
        .unwrap_or(1.0);

    Some(decimal * sign)
}
