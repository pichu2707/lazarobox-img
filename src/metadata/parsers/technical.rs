use crate::metadata::ImageMetadata;
use crate::metadata::parsers::set_string_field;

pub fn parse(exif_data: &exif::Exif, metadata: &mut ImageMetadata) {
    for field in exif_data.fields() {
        if field.tag == exif::Tag::Orientation {
            set_string_field(
                &mut metadata.technical.orientation,
                field.display_value().with_unit(exif_data).to_string(),
            );
        }
    }
}
