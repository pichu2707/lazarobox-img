//Metadata de imágenes sobre IA.
use crate::metadata::MetadataField;
#[derive(Debug, Clone, Default)]
pub struct AiMetadata {
    pub detected: bool,
    pub software: MetadataField<String>,
    pub model: MetadataField<String>,
    pub license: MetadataField<String>,
    pub source_url: MetadataField<String>,
}
