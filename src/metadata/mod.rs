//! Motor de metadatos de LazaroBox Image.
//!
//! Este módulo agrupa la lectura, modelado y futura escritura de metadatos
//! asociados a imágenes.
//!
//! Responsabilidades:
//! - Leer metadatos existentes.
//! - Mapear EXIF/IPTC/XMP al modelo interno.
//! - Representar información web, SEO, derechos, IA y técnica.
//!
//! Este módulo no debe optimizar imágenes ni gestionar rutas de salida.
pub mod ai;
pub mod editor;
pub mod image;
pub mod parsers;
pub mod plan;
pub mod privacy;
pub mod reader;
pub mod report;
pub mod rights;
pub mod seo;
pub mod social;
pub mod technical;
pub mod types;
pub mod web;
pub mod writer;

pub use image::ImageMetadata;
pub use reader::read_metadata;
pub use types::{MetadataField, MetadataStatus};
