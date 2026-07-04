//Metadata de imágenes sobre IA.

#[derive(Debug, Clone, Default)]
pub struct Name {
    pub detected: bool,
    pub software: MetadataField<String>,
    pub model: MetadataField<String>,
    pub license: MetadataField<String>,
    pub source_url: MetadataField<String>,
}
