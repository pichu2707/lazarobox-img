use crate::theme;
use crate::types::ProgressState;

pub fn print(progress: &ProgressState) {
    theme::box_start("Progress");

    theme::key_value("Stage", &progress.stage);
    theme::key_value(
        "Progress",
        &format!("{}/{}", progress.current, progress.total),
    );
    theme::key_value("Current", &progress.filename);

    theme::box_end();
}
