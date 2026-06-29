mod scanner;
mod optimizer;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name= "lazarobox-img",
    version="0.1.0",
    about="Optimizador de imágenes CLI con estilo LazaroBox"
)]

struct Args {
    /// Imagen o carpeta a optimizar.
    input: PathBuf,

    /// Ancho máximo de salida
    #[arg(short, long, default_value_t = 1200)]
    width: u32,

    /// Calidad de salida, de 1 a 100
    #[arg(short, long, default_value_t = 80)]
    quality: u8,

    /// Formato de salida: webp, jpg o png
    #[arg(short, long, default_value = "webp")]
    format: String,
}


fn main()->anyhow::Result<()> {
    let args = Args::parse();

    println!("LazaroBox Image Optimizer");
    println!("Entrada {:?}", args.input);
    println!("Ancho máximo: {}", args.width);
    println!("Calidad: {}", args.quality);
    println!("Formato: {}", args.format);

    let images = scanner::find_images(&args.input)?;
    println!();
    println!("Imagen destacadas: {}", images.len());

    for image in images {
        let info = optimizer::inspect(&image)?;
        println!("- {:#?}", info);
    }
    Ok(())
}
