use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::AppState;
use crate::tui::screens::common;

pub fn render(frame: &mut Frame, state: &AppState) {
    let inner = common::screen_frame(frame, "Inspeccionar imagen");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // título
            Constraint::Length(1), // separador
            Constraint::Min(1),    // navegador o resultado
            Constraint::Length(1), // pie
        ])
        .split(inner);

    let heading = Paragraph::new(Line::from("Inspeccionar imagen"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();
    frame.render_widget(heading, chunks[0]);

    if state.inspect.viewing {
        if state.inspect.pending.is_some() {
            frame.render_widget(common::processing(), chunks[2]);
        } else {
            frame.render_widget(result_body(state), chunks[2]);
        }
        frame.render_widget(footer("Enter / Esc  Volver al navegador"), chunks[3]);
    } else {
        common::render_browser(frame, chunks[2], &state.browser);
        frame.render_widget(
            footer("↑↓ Mover   Enter Abrir/Elegir   Backspace Subir   Esc Volver"),
            chunks[3],
        );
    }
}

fn footer(text: &str) -> Paragraph<'_> {
    Paragraph::new(Line::from(text)).style(Style::default().fg(Color::DarkGray))
}

/// Resultado de la última inspección, o el error.
fn result_body(state: &AppState) -> Paragraph<'static> {
    if let Some(error) = &state.inspect.error {
        return Paragraph::new(Line::from(Span::styled(
            format!("✖ {error}"),
            Style::default().fg(Color::Red),
        )));
    }

    if let Some(info) = &state.inspect.result {
        let lines = vec![
            info_line("Archivo", info.filename.clone()),
            info_line("Dimensiones", format!("{} × {} px", info.width, info.height)),
            info_line("Tamaño", common::human_size(info.size)),
            info_line("Ruta", info.path.display().to_string()),
        ];
        return Paragraph::new(lines);
    }

    Paragraph::new(Line::from(""))
}

/// Línea "etiqueta      valor" con la columna de etiqueta de ancho fijo.
fn info_line(label: &str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<12}"), Style::default().fg(Color::Gray)),
        Span::styled(value, Style::default().fg(Color::White)),
    ])
}
