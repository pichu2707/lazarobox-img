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
    frame.render_widget(footer(footer_text(state)), chunks[3]);
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
    lines.extend(status_lines(state));

    if let Some(log) = &update.log {
        lines.push(Line::from(""));
        for line in log.lines() {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Gray),
            )));
        }
    }

    Paragraph::new(lines)
}

fn status_lines(state: &AppState) -> Vec<Line<'static>> {
    match state.update.status {
        UpdateStatus::NotChecked => vec![Line::from(Span::styled(
            "Pulsa c para comprobar si hay una actualización disponible.",
            Style::default().fg(Color::Gray),
        ))],
        UpdateStatus::Checking => vec![Line::from(Span::styled(
            "Comprobando actualización…",
            Style::default().fg(Color::Yellow),
        ))],
        UpdateStatus::UpToDate => vec![Line::from(Span::styled(
            "✓ Estás usando la última versión estable.",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))],
        UpdateStatus::UpdateAvailable => vec![Line::from(Span::styled(
            "⚠ Hay una actualización disponible.",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))],
        UpdateStatus::Confirming => vec![
            Line::from(Span::styled(
                "Confirma la actualización por Homebrew.",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "Se ejecutarán estos comandos, sin sudo y sin Cargo:",
                Style::default().fg(Color::Gray),
            )),
        ],
        UpdateStatus::Updating => vec![Line::from(Span::styled(
            "Actualizando…",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))],
        UpdateStatus::Updated => vec![Line::from(Span::styled(
            "✓ Actualización completada. Reinicia la app para usar la nueva versión.",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))],
        UpdateStatus::Error => vec![Line::from(Span::styled(
            format!(
                "✖ No se pudo completar la actualización: {}",
                state.update.error.as_deref().unwrap_or("error desconocido")
            ),
            Style::default().fg(Color::Red),
        ))],
    }
}

fn footer_text(state: &AppState) -> &'static str {
    match state.update.status {
        UpdateStatus::UpdateAvailable => "u Actualizar con Homebrew   c Comprobar   Esc Volver",
        UpdateStatus::Confirming => "Enter Confirmar   Esc Cancelar",
        UpdateStatus::Updating => "Actualizando…",
        _ => "c Comprobar actualización   Esc Volver",
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
