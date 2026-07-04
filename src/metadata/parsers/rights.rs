use crate::metadata::ImageMetadata;
use crate::metadata::parsers::set_string_field;

pub fn parse(exif_data: &exif::Exif, metadata: &mut ImageMetadata) {
    for field in exif_data.fields() {
        match field.tag {
            exif::Tag::Artist => {
                set_string_field(
                    &mut metadata.rights.author,
                    field.display_value().with_unit(exif_data).to_string(),
                );
            }

            exif::Tag::Copyright => {
                set_string_field(
                    &mut metadata.rights.copyright,
                    field.display_value().with_unit(exif_data).to_string(),
                );
            }

            _ => {}
        }
    }
}
