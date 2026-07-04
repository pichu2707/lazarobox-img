use anyhow::Result;
use std::path::Path;

use crate::inspector;
use crate::types::OutputFormat;

#[derive(Debug)]
pub enum OptimizationDecision {
    Optimize,
    SkipAlreadyOptimized,
    SkipTooSmall,
    SkipSameFormat,
}

pub fn evaluate(
    image: &Path,
    format: OutputFormat,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<OptimizationDecision> {
    let same_format = image
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case(format.extension()))
        .unwrap_or(false);

    let info = inspector::inspect(&image)?;

    let already_small_enough = width.map(|w| info.width <= w).unwrap_or(true)
        && height.map(|h| info.height <= h).unwrap_or(true);

    if same_format && already_small_enough {
        return Ok(OptimizationDecision::SkipAlreadyOptimized);
    }
    Ok(OptimizationDecision::Optimize)
}
