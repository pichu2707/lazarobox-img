#[derive(Debug, Clone)]
pub struct ProgressState {
    pub stage: String,
    pub current: usize,
    pub total: usize,
    pub filename: String,
}

impl ProgressState {
    pub fn new(stage: &str, current: usize, total: usize, filename: &str) -> Self {
        Self {
            stage: stage.to_string(),
            current,
            total,
            filename: filename.to_string(),
        }
    }
}
