#[derive(Debug, Clone, Default)]
pub enum MetadataStatus {
    #[default]
    /// El campo no existe en la imagen.
    Missing,
    /// El campo existe y no ha cambiado.
    Present,
    /// El campo ha sido añadido por la herramienta.
    Added,
    /// El valor ha sido modificado.
    Modified,
    /// El campo existía y ha sido eliminado.
    Removed,
}

#[derive(Debug, Clone)]
pub struct MetadataField<T> {
    pub value: Option<T>,
    pub status: MetadataStatus,
}

impl<T> Default for MetadataField<T> {
    fn default() -> Self {
        MetadataField {
            value: None,
            status: MetadataStatus::Missing,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImageMetadata {
    pub latitude: MetadataField<f64>,
    pub longitude: MetadataField<f64>,

    pub alt_text: MetadataField<String>,
    pub copyright: MetadataField<String>,
    pub ai_metadata: MetadataField<String>,
}
