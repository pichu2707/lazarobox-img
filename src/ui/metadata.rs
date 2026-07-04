use crate::metadata::{ImageMetadata, MetadataField};

fn print_field(label: &str, field: &MetadataField<String>) {
    match &field.value {
        Some(value) => println!("│  {}: ✓ {}", label, value),
        None => println!("│  {}: ✗", label),
    }
}

pub fn print(metadata: &ImageMetadata) {
    println!("╭─ Metadata");

    println!("│ WEB");
    print_field("Description", &metadata.web.description);
    print_field("Alt text", &metadata.web.alt_text);

    println!("│");
    println!("│ RIGHTS");
    print_field("Author", &metadata.rights.author);
    print_field("Copyright", &metadata.rights.copyright);

    println!("│");
    println!("│ AI");
    print_field("Software", &metadata.ai.software);

    println!("╰────────────────────────────────────");
}
