mod encoders;
mod export;
mod inspector;
mod metadata;
mod optimizer;
mod policy;
mod report;
mod scanner;
mod types;

use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;
use types::OutputFormat;

use crate::report::print_result;

#[derive(Parser, Debug)]
#[command(
    name = "lazarobox-img",
    version = "0.1.0",
    about = "Optimizador de imágenes CLI con estilo LazaroBox"
)]

struct Args {
    /// Imagen o carpeta a optimizar.
    input: PathBuf,

    /// Ancho máximo de salida
    #[arg(short, long)]
    width: Option<u32>,

    #[arg(long)]
    height: Option<u32>,

    /// Calidad de salida, de 1 a 100
    #[arg(short, long, default_value_t = 80)]
    quality: u8,

    /// Formato de salida: webp, jpg o png
    #[arg(short, long, default_value_t = OutputFormat::Webp)]
    format: OutputFormat,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let start = Instant::now();

    println!("LazaroBox Image Optimizer");
    println!("Entrada {:?}", args.input);
    println!("Ancho máximo: {:?}", args.width);
    println!("Alto máximo: {:?}", args.height);
    println!("Calidad: {}", args.quality);
    println!("Formato: {}", args.format);

    let images = scanner::find_images(&args.input)?;
    println!();
    println!("Imagen destacadas: {}", images.len());

    let output_dir = export::create_output_dir(&args.input)?;
    println!("Directorioro de salida: {}", output_dir.display());
    let mut optimization_results = Vec::new();
    let mut skipped = 0usize;

    for image in images {
        let output_file = export::create_output_file(&output_dir, &image, args.format.extension())?;

        let result = optimizer::optimize(
            &image,
            &output_file,
            args.width,
            args.height,
            args.quality,
            args.format,
        )?;
        report::print_result(&result);
        optimization_results.push(result);
        let metadata = metadata::read_metadata(&image)?;
        println!("{:?}", metadata);
        match policy::evaluate(&image, args.format, args.width, args.height)? {
            policy::OptimizationDecision::SkipAlreadyOptimized => {
                println!("Saltada: {} ya está optimizada: ", image.display());
                skipped += 1;
                continue;
            }
            policy::OptimizationDecision::Optimize => {}
            _ => {}
        }
    }

    let duration = start.elapsed();

    report::print_summary(&optimization_results, skipped, &output_dir, duration);
    Ok(())
}
