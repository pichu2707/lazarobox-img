// Metadata de imágenes para privacidad

#[derive(Debug, Clone, Default)]
pub struct Name {
    pub has_gps: bool,
    pub has_camera_serial: bool,
    pub has_personal_data: bool,
}
