//Datos de imágenes optimzidas para redes sociales
use crate::metadata::MetadataField;
#[derive(Debug, Clone, Default)]
pub struct SocialMetadata {
    pub og_title: MetadataField<String>,
    pub og_description: MetadataField<String>,
    pub campaign: MetadataField<String>,
}
