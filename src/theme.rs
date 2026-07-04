//! Lenguaje visual de LazaroBox para salida por terminal.

pub const CHECK: &str = "✓";
pub const CROSS: &str = "✗";
pub const ARROW: &str = "›";
pub const WARNING: &str = "⚠";
pub const LINE: &str = "────────────────────────────────────";

pub fn title(title: &str, subtitle: &str, version: &str) {
    println!("╭────────────────────────────────────╮");
    println!("│{:^36}│", "");
    println!("│{:^36}│", title);
    println!("│{:^36}│", subtitle);
    println!("│{:^36}│", version);
    println!("│{:^36}│", "");
    println!("╰────────────────────────────────────╯");
}

pub fn box_start(title: &str) {
    println!();
    println!("╭─ {}", title);
}

pub fn box_end() {
    println!("╰{}", LINE);
}

pub fn empty_line() {
    println!("│");
}

pub fn key_value(label: &str, value: &str) {
    println!("│ {:<16} {}", label, value);
}

pub fn success(label: &str, value: &str) {
    println!("│ {:<16} {} {}", label, CHECK, value);
}

pub fn missing(label: &str) {
    println!("│ {:<16} {}", label, CROSS);
}

pub fn warning(label: &str, value: &str) {
    println!("│ {:<16} {} {}", label, WARNING, value);
}

pub fn stage(label: &str) {
    println!();
    println!("{} {}", ARROW, label);
}
