// Metadata de derecho de la imagen

#[derive(Debug, Clone, Default)]
pub struct RightsMetadata {
    pub author: MetadataField<String>,
    pub copyright: MetadataField<String>,
    pub license: MetadataField<String>,
    pub source_url: MetadataField<String>,
}
