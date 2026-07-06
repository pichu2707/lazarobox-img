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
    let inner = common::screen_frame(frame, "Convertir formatos");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // título
            Constraint::Length(1), // formato de destino
            Constraint::Length(1), // separador
            Constraint::Min(1),    // navegador o resultado
            Constraint::Length(1), // pie
        ])
        .split(inner);

    let heading = Paragraph::new(Line::from("Convertir formatos"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();
    frame.render_widget(heading, chunks[0]);

    let format = Paragraph::new(Line::from(vec![
        Span::styled("Formato destino: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("◄ {} ►", state.convert.format.extension().to_uppercase()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    frame.render_widget(format, chunks[1]);

    if state.convert.viewing {
        if state.convert.pending.is_some() {
            frame.render_widget(common::processing(), chunks[3]);
        } else {
            frame.render_widget(result_body(state), chunks[3]);
        }
        frame.render_widget(footer("Enter / Esc  Volver al navegador"), chunks[4]);
    } else {
        common::render_browser(frame, chunks[3], &state.browser);
        frame.render_widget(
            footer("↑↓ Mover   ←→ Formato   Enter Abrir/Convertir   Backspace Subir   Esc Volver"),
            chunks[4],
        );
    }
}

fn footer(text: &str) -> Paragraph<'_> {
    Paragraph::new(Line::from(text)).style(Style::default().fg(Color::DarkGray))
}

/// Resultado de la última conversión, o el error.
fn result_body(state: &AppState) -> Paragraph<'static> {
    if let Some(error) = &state.convert.error {
        return Paragraph::new(Line::from(Span::styled(
            format!("✖ {error}"),
            Style::default().fg(Color::Red),
        )));
    }

    if let Some(conversion) = &state.convert.result {
        let lines = vec![
            Line::from(Span::styled(
                "✔ Convertida",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("Salida: {}", conversion.output_path.display())),
            Line::from(format!("Tamaño: {}", common::human_size(conversion.size))),
        ];
        return Paragraph::new(lines);
    }

    Paragraph::new(Line::from(""))
}
