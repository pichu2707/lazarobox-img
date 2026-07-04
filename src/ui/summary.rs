use crate::optimizer::OptimizationResult;
use std::{path::Path, time::Duration};

use super::utils::{bytes_to_mb, saving_percent};

pub fn print_summary(
    results: &[OptimizationResult],
    skipped: usize,
    output_dir: &Path,
    duration: Duration,
) {
    let processed = results.len();

    let original_total: u64 = results.iter().map(|result| result.original.size).sum();

    let optimized_total: u64 = results
        .iter()
        .flat_map(|result| result.outputs.iter())
        .map(|output| output.size)
        .sum();

    println!();
    println!("╭─ Resumen final");
    println!("│ Imágenes optimizadas: {}", processed);
    println!(
        "│ Peso original total: {:.2} MB",
        bytes_to_mb(original_total)
    );
    println!(
        "│ Peso optimizado total: {:.2} MB",
        bytes_to_mb(optimized_total)
    );
    println!(
        "│ Ahorro total: {:.2}%",
        saving_percent(original_total, optimized_total)
    );
    println!("│ Imágenes saltadas: {}", skipped);
    println!("│ Tiempo total: {:.2?}", duration);
    println!("│ Carpeta de salida: {}", output_dir.display());
    println!("╰────────────────────────────────────");
}
