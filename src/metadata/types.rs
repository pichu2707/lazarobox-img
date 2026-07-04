#[derive(Debug, Clone, Default)]
pub enum MetadataStatus {
    #[default]
    Missing,
    Present,
    Added,
    Modified,
    Removed,
    Preserved,
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
