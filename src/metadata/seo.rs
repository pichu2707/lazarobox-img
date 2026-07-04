// Datos de imágenes optimizado para SEO
pub struct SeoMetadata {
    pub latitude: MetadataField<f64>,
    pub longitude: MetadataField<f64>,

    pub business_name: MetadataField<String>,
    pub location_name: MetadataField<String>,
    pub service_name: MetadataField<String>,
    pub ai_generated_alt: MetadataField<String>,
}
