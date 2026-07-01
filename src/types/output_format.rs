use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Webp,
    Avif,
    Jpeg,
    Png,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Webp => "webp",
            OutputFormat::Avif => "avif",
            OutputFormat::Jpeg => "jpg",
            OutputFormat::Png => "png",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.extension())
    }
}
