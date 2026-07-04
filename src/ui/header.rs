use crate::theme;
use crate::types::OutputFormat;
use std::path::Path;

pub fn print(
    input: &Path,
    width: Option<u32>,
    height: Option<u32>,
    quality: u8,
    format: OutputFormat,
) {
    theme::section("LZB Image Optimizer");

    theme::line("Entrada", &input.display().to_string());
    theme::line("Ancho máximo", &format!("{:?}", width));
    theme::line("Alto máximo", &format!("{:?}", height));
    theme::line("Calidad", &quality.to_string());
    theme::line("Formato", &format.to_string());

    theme::end_section();
}
