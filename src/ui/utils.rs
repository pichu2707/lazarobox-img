pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0
}

pub fn saving_percent(original: u64, optimized: u64) -> f64 {
    if original == 0 {
        return 0.0;
    }

    100.0 - ((optimized as f64 / original as f64) * 100.0)
}
