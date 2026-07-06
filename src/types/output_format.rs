use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Webp,
    Avif,
    Jpeg,
    Png,
}

impl OutputFormat {
    /// Todos los formatos, en orden de selección.
    pub const ALL: [OutputFormat; 4] = [
        OutputFormat::Webp,
        OutputFormat::Avif,
        OutputFormat::Jpeg,
        OutputFormat::Png,
    ];

    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Webp => "webp",
            OutputFormat::Avif => "avif",
            OutputFormat::Jpeg => "jpg",
            OutputFormat::Png => "png",
        }
    }

    /// Siguiente formato en `ALL`, con wrap.
    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|&f| f == self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    /// Formato anterior en `ALL`, con wrap.
    pub fn previous(self) -> Self {
        let index = Self::ALL.iter().position(|&f| f == self).unwrap_or(0);
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.extension())
    }
}
