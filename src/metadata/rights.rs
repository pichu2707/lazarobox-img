// Metadata de derecho de la imagen
use crate::metadata::MetadataField;
#[derive(Debug, Clone, Default)]
pub struct RightsMetadata {
    pub author: MetadataField<String>,
    pub copyright: MetadataField<String>,
    pub license: MetadataField<String>,
    pub source_url: MetadataField<String>,
}
