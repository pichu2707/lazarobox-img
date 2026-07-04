// Datos de imagen para web.
use crate::metadata::MetadataField;
#[derive(Debug, Clone, Default)]
pub struct WebMetadata {
    pub description: MetadataField<String>,
    pub alt_text: MetadataField<String>,
    pub keywords: MetadataField<String>,
}
