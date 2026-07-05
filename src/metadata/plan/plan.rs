use super::MetadataAction;

#[derive(Debug, Clone)]
pub struct MetadataPlan {
    pub ai_software: MetadataAction,
    pub ai_model: MetadataAction,
    pub ai_license: MetadataAction,
    pub ai_source_url: MetadataAction,

    pub gps: MetadataAction,

    pub author: MetadataAction,
    pub copyright: MetadataAction,

    pub description: MetadataAction,
    pub alt_text: MetadataAction,
}
