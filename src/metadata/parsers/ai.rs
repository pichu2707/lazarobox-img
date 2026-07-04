use crate::metadata::ImageMetadata;
use crate::metadata::parsers::set_string_field;

pub fn parse(exif_data: &exif::Exif, metadata: &mut ImageMetadata) {
    for field in exif_data.fields() {
        if field.tag == exif::Tag::Software {
            let value = field.display_value().with_unit(exif_data).to_string();

            let lower = value.to_lowercase();

            if lower.contains("midjourney")
                || lower.contains("stable diffusion")
                || lower.contains("comfyui")
                || lower.contains("flux")
                || lower.contains("dall")
                || lower.contains("fooocus")
            {
                metadata.ai.detected = true;
            }

            set_string_field(&mut metadata.ai.software, value);
        }
    }
}
