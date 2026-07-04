use crate::metadata::{
    ai::AiMetadata, privacy::PrivacyMetadata, rights::RightsMetadata, seo::SeoMetadata,
    social::SocialMetadata, technical::TechnicalMetadata, web::WebMetadata,
};

#[derive(Debug, Clone, Default)]
pub struct ImageMetadata {
    pub web: WebMetadata,
    pub seo: SeoMetadata,
    pub social: SocialMetadata,
    pub privacy: PrivacyMetadata,
    pub ai: AiMetadata,
    pub rights: RightsMetadata,
    pub technical: TechnicalMetadata,
}
