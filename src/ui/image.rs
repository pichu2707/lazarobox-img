use crate::optimizer::OptimizationResult;

use super::utils::{bytes_to_mb, saving_percent};

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
