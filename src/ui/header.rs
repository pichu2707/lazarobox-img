use crate::theme;
use crate::types::OutputFormat;
use std::path::Path;

pub fn print() {
    theme::title(
        "LZB Image Optimizer",
        "Optimización & Metadata Toolkit",
        env!("CARGO_PKG_VERSION"),
    );
}
