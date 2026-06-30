use crate::optimizer::OptimizationResult;
use std::{path::Path, time::Duration};

/// Convierte bytes a MB.
fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0
}

/// Calcula el porcentaje de ahorro entre original y optimizado.
fn saving_percent(original: u64, optimized: u64) -> f64 {
    if original == 0 {
        return 0.0;
    }

    100.0 - ((optimized as f64 / original as f64) * 100.0)
}

/// Muestra el resultado de optimizar una imagen.
///
/// De momento usa `println!`.
/// Más adelante esta capa podrá evolucionar a Ratatui.
pub fn print_result(result: &OptimizationResult) {
    println!();
    println!("| {}", result.original.filename);
    println!("| original");
    println!("| Peso: {:.2} MB", bytes_to_mb(result.original.size));
    println!(
        "| Resolución: {}x{}",
        result.original.width, result.original.height
    );

    for output in &result.outputs {
        println!();
        println!("╭─  optimizada ({})", output.format);
        println!("| Archivo: {}", output.output_path.display());
        println!("| Peso {:.2} MB", bytes_to_mb(output.size));
        println!("| Resolución: {}x{}", output.width, output.height);
        println!(
            "| Ahorro: {:.2}%",
            saving_percent(result.original.size, output.size)
        );
    }
    println!("╰────────────────────────────────────");
}

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
