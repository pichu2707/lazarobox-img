use crate::metadata::ImageMetadata;

use super::{MetadataAction, MetadataPlan};
pub fn build_remove_ai_plan(metadata: &ImageMetadata) -> MetadataPlan {
    MetadataPlan {
        ai_software: MetadataAction::Remove,
        ai_model: MetadataAction::Remove,
        ai_license: MetadataAction::Remove,
        ai_source_url: MetadataAction::Remove,

        gps: MetadataAction::Preserve,

        author: MetadataAction::Preserve,
        copyright: MetadataAction::Preserve,

        description: MetadataAction::Preserve,
        alt_text: MetadataAction::Preserve,
    }
}
