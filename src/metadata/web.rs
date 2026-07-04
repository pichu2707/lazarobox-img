// Datos de imagen para web.

#[derive(Debug, Clone, Default)]
pub struct WebMetadata {
    pub description: MetadataField<String>,
    pub alt_text: MetadataField<String>,
    pub keywords: MetadataField<String>,
}
