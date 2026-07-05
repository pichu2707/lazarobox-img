use std::path::Path;

use crate::theme;

pub fn print_output(path: &Path) {
    theme::box_start("Metadata Writer");
    theme::success("Status", "Copied");
    theme::key_value("Output", &path.display().to_string());
    theme::box_end();
}
