use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::{AppState, MenuItem},
    tui::widgets,
};

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    let outer = Block::default()
        .title(" LazaroBox Image ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(outer, area);

    let inner = area.inner(Margin {
        vertical: 1,
        horizontal: 2,
    });

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // título
            Constraint::Length(1), // subtítulo
            Constraint::Min(1),    // oso + menú
            Constraint::Length(1), // ayuda contextual
            Constraint::Length(1), // pie
        ])
        .split(inner);

    let title = Paragraph::new(Line::from("LazaroBox Image"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();

    let subtitle = Paragraph::new(Line::from("Optimización de imágenes y metadatos"))
        .style(Style::default().fg(Color::Gray))
        .centered();

    frame.render_widget(title, rows[0]);
    frame.render_widget(subtitle, rows[1]);

    // Zona central: el oso a la izquierda, un hueco y el menú a la derecha.
    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(40),
            Constraint::Length(2),
            Constraint::Length(26),
        ])
        .split(rows[2]);

    widgets::logo::render(frame, middle[0]);

    // Menú centrado verticalmente en su columna (6 ítems).
    let menu_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(6),
            Constraint::Min(1),
        ])
        .split(middle[2]);

    widgets::menu::render(frame, menu_rows[1], state.selected);

    // Ayuda contextual: describe el ítem resaltado.
    let help = MenuItem::ALL
        .get(state.selected)
        .map(|item| item.description())
        .unwrap_or_default();
    let help = Paragraph::new(Line::from(help))
        .style(Style::default().fg(Color::Gray))
        .centered();
    frame.render_widget(help, rows[3]);

    widgets::footer::render(frame, rows[4]);
}
