pub const CHECK: &str = "✓";
pub const CROSS: &str = "✗";

pub fn section(title: &str) {
    println!("╭─ {}", title);
}

pub fn end_section() {
    println!("╰────────────────────────────────────");
}

pub fn line(label: &str, value: &str) {
    println!("│ {}: {}", label, value);
}

pub fn success(label: &str, value: &str) {
    println!("│ {}: {} {}", label, CHECK, value);
}

pub fn missing(label: &str) {
    println!("│ {}: {}", label, CROSS);
}
