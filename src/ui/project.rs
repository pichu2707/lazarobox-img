use crate::theme;
use crate::types::OutputFormat;

use std::path::Path;

pub fn print(
    input: &Path,
    output: &Path,
    images: usize,
    width: Option<u32>,
    height: Option<u32>,
    quality: u8,
    format: OutputFormat,
) {
    theme::box_start("Project");

    theme::key_value("Input", &input.display().to_string());
    theme::key_value("Output", &output.display().to_string());
    theme::key_value("Images", &images.to_string());

    theme::empty_line();

    theme::key_value("Width", &format!("{:?}", width));
    theme::key_value("Height", &format!("{:?}", height));
    theme::key_value("Quality", &quality.to_string());
    theme::key_value("Format", &format.to_string());

    theme::box_end();
}
