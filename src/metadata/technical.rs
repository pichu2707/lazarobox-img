// Metadata de imágenes optimizada para imágenes técniccas
use crate::metadata::MetadataField;
#[derive(Debug, Clone, Default)]
pub struct TechnicalMetadata {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub orientation: MetadataField<String>,
    pub color_profile: MetadataField<String>,
}
