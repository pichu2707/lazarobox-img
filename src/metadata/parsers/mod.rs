pub mod ai;
pub mod rights;
pub mod seo;
pub mod technical;
pub mod web;

use crate::metadata::{ImageMetadata, MetadataField, MetadataStatus};

pub fn set_string_field(field: &mut MetadataField<String>, value: String) {
    let clean = normalize_string(&value);

    if clean.is_empty() {
        return;
    }

    field.value = Some(clean);
    field.status = MetadataStatus::Present;
}

pub fn parse_all(exif_data: &exif::Exif, metadata: &mut ImageMetadata) {
    web::parse(exif_data, metadata);
    rights::parse(exif_data, metadata);
    ai::parse(exif_data, metadata);
    technical::parse(exif_data, metadata);
    seo::parse(exif_data, metadata);
}

fn normalize_string(value: &str) -> String {
    value
        .replace("\\x0a", "\n")
        .replace('\r', "")
        .trim()
        .trim_matches('"')
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
