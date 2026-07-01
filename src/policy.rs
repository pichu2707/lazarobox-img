pub enum OptimizationDecision {
    Optimize,
    SkipAlreadyOptimized,
    SkipTooSmall,
    SkipSameFormat,
}

let same_format = image
.extension()
.and_then(|ext|ext.to_str())
.map(|ext|ext.eq_ignore_ascii_case(&args.format))
.unwrap_or(false);

let info = inspector::inspect(&image)?;

let already_small_enough =
    args.width.map(|w| info.width <= w).unwrap_or(true)
    &&
    args.height.map(|h| info.height <= h).unwrap_or(true);

if same_format && already_small_enough {
    println!("Saltada: {} ya está optimizada", image.display());
    skipped += 1;
    continue;
}
