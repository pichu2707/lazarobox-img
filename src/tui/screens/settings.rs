use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::AppState;
use crate::tui::screens::common;
use crate::update::UpdateStatus;

pub fn render(frame: &mut Frame, state: &AppState) {
    let inner = common::screen_frame(frame, "Ajustes");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // título
            Constraint::Length(1), // separador
            Constraint::Min(1),    // contenido
            Constraint::Length(1), // pie
        ])
        .split(inner);

    let heading = Paragraph::new(Line::from("Ajustes"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();
    frame.render_widget(heading, chunks[0]);

    frame.render_widget(body(state), chunks[2]);
    frame.render_widget(footer("c Comprobar actualización   Esc Volver"), chunks[3]);
}

fn body(state: &AppState) -> Paragraph<'static> {
    let update = &state.update;
    let mut lines = vec![info_line(
        "Versión actual",
        update.current_version.to_string(),
    )];

    if let Some(latest) = &update.latest_version {
        lines.push(info_line("Última versión", latest.clone()));
    }

    lines.push(Line::from(""));
    lines.push(status_line(state));

    Paragraph::new(lines)
}

fn status_line(state: &AppState) -> Line<'static> {
    match state.update.status {
        UpdateStatus::NotChecked => Line::from(Span::styled(
            "Pulsa c para comprobar si hay una actualización disponible.",
            Style::default().fg(Color::Gray),
        )),
        UpdateStatus::Checking => Line::from(Span::styled(
            "Comprobando actualización…",
            Style::default().fg(Color::Yellow),
        )),
        UpdateStatus::UpToDate => Line::from(Span::styled(
            "✓ Estás usando la última versión estable.",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        UpdateStatus::UpdateAvailable => Line::from(Span::styled(
            "⚠ Hay una actualización disponible.",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        UpdateStatus::Error => Line::from(Span::styled(
            format!(
                "✖ No se pudo comprobar la actualización: {}",
                state.update.error.as_deref().unwrap_or("error desconocido")
            ),
            Style::default().fg(Color::Red),
        )),
    }
}

fn info_line(label: &str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<16}"), Style::default().fg(Color::Gray)),
        Span::styled(value, Style::default().fg(Color::White)),
    ])
}

fn footer(text: &str) -> Paragraph<'_> {
    Paragraph::new(Line::from(text)).style(Style::default().fg(Color::DarkGray))
}
