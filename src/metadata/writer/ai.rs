use crate::metadata::{
    ImageMetadata, MetadataStatus,
    plan::{MetadataAction, MetadataPlan},
};

pub fn apply(metadata: &mut ImageMetadata, plan: &MetadataPlan) {
    if matches!(plan.ai_software, MetadataAction::Remove) {
        metadata.ai.software.value = None;
        metadata.ai.software.status = MetadataStatus::Removed;
    }

    if matches!(plan.ai_model, MetadataAction::Remove) {
        metadata.ai.model.value = None;
        metadata.ai.model.status = MetadataStatus::Removed;
    }

    if matches!(plan.ai_license, MetadataAction::Remove) {
        metadata.ai.license.value = None;
        metadata.ai.license.status = MetadataStatus::Removed;
    }

    if matches!(plan.ai_source_url, MetadataAction::Remove) {
        metadata.ai.source_url.value = None;
        metadata.ai.source_url.status = MetadataStatus::Removed;
    }
}
