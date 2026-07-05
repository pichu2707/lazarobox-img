use crate::metadata::plan::{MetadataAction, MetadataPlan};
use crate::theme;

fn print_action(label: &str, action: MetadataAction) {
    let value = match action {
        MetadataAction::Preserve => "Preserve",
        MetadataAction::Add => "Add",
        MetadataAction::Modify => "Modify",
        MetadataAction::Remove => "Remove",
    };

    theme::key_value(label, value);
}

pub fn print(plan: &MetadataPlan) {
    theme::box_start("Metadata Plan");

    print_action("AI software", plan.ai_software);
    print_action("AI model", plan.ai_model);
    print_action("AI license", plan.ai_license);
    print_action("AI source URL", plan.ai_source_url);

    theme::empty_line();

    print_action("GPS", plan.gps);
    print_action("Author", plan.author);
    print_action("Copyright", plan.copyright);
    print_action("Description", plan.description);
    print_action("Alt text", plan.alt_text);

    theme::box_end();
}
