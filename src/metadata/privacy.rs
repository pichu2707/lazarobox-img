// Metadata de imágenes para privacidad
use crate::metadata::MetadataField;
#[derive(Debug, Clone, Default)]
pub struct PrivacyMetadata {
    pub has_gps: bool,
    pub has_camera_serial: bool,
    pub has_personal_data: bool,
}
