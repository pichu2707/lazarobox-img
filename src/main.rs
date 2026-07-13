mod app;
mod converter;
mod encoders;
mod export;
mod inspector;
mod metadata;
mod optimizer;
mod policy;
mod scanner;
mod theme;
mod tui;
mod types;
mod ui;
mod update;

use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;
use types::OutputFormat;

#[derive(Parser, Debug)]
#[command(
    name = "lazarobox-img",
    version = env!("CARGO_PKG_VERSION"),
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
    if std::env::args().len() == 1 {
        let mut terminal = tui::terminal::init_terminal()?;
        let mut controller = app::AppController::new();

        let result = tui::app::run(&mut terminal, &mut controller);

        tui::terminal::restore_terminal(terminal)?;

        return result;
    }
    let args = Args::parse();

    ui::header::print();

    let start = Instant::now();

    let images = scanner::find_images(&args.input)?;
    println!();
    println!("Imagen destacadas: {}", images.len());

    let output_dir = export::create_output_dir(&args.input)?;
    println!("Directorioro de salida: {}", output_dir.display());
    ui::project::print(
        &args.input,
        &output_dir,
        images.len(),
        args.width,
        args.height,
        args.quality,
        args.format,
    );

    let mut optimization_results = Vec::new();
    let mut skipped = 0usize;

    for (index, image) in images.iter().enumerate() {
        let progress = types::ProgressState::new(
            "Processing image",
            index + 1,
            images.len(),
            &image.display().to_string(),
        );
        let metadata = metadata::read_metadata(image)?;
        // presentacion::console::metadata::print(&metadata);
        ui::metadata::print(&metadata);
        ui::progress::print(&progress);
        let metadata_plan = metadata::plan::build_remove_ai_plan(&metadata);
        ui::plan::print(&metadata_plan);

        let mut cleaned_metadata = metadata.clone();
        metadata::writer::apply(&mut cleaned_metadata, &metadata_plan);
        theme::stage("After applying metadata plan");
        ui::metadata::print(&cleaned_metadata);

        match policy::evaluate(image, args.format, args.width, args.height)? {
            policy::OptimizationDecision::SkipAlreadyOptimized => {
                println!("Saltada: {} ya está optimizada: ", image.display());
                skipped += 1;
                continue;
            }
            policy::OptimizationDecision::Optimize => {}
            _ => {}
        }

        let output_file = export::create_output_file(&output_dir, image, args.format.extension())?;

        let result = optimizer::optimize(
            image,
            &output_file,
            args.width,
            args.height,
            args.quality,
            args.format,
        )?;

        ui::image::print_result(&result);
        optimization_results.push(result);
    }

    let duration = start.elapsed();

    ui::summary::print_summary(&optimization_results, skipped, &output_dir, duration);
    Ok(())
}
