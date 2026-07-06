use ratatui::Frame;

use crate::app::AppState;
use crate::tui::screens::common;

pub fn render(frame: &mut Frame, _state: &AppState) {
    common::render_placeholder(
        frame,
        "Ajustes",
        "Configura el formato, la calidad y la política de metadatos por defecto. (Próximamente)",
    );
}
