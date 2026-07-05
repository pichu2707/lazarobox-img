use crate::metadata::{ImageMetadata, plan::MetadataPlan};

use super::{ai, rights, seo, technical};

pub fn apply(metadata: &mut ImageMetadata, plan: &MetadataPlan) {
    ai::apply(metadata, plan);
    rights::apply(metadata, plan);
    seo::apply(metadata, plan);
    technical::apply(metadata, plan);
}
