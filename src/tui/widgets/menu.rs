use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::MenuItem;

pub fn render(frame: &mut Frame, area: Rect, selected: usize) {
    let lines: Vec<Line> = MenuItem::ALL
        .iter()
        .enumerate()
        .map(|(index, item)| {
            if index == selected {
                Line::from(vec![Span::styled(
                    format!("▶ {}", item.label()),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )])
            } else {
                Line::from(format!("  {}", item.label()))
            }
        })
        .collect();

    let widget = Paragraph::new(lines);

    frame.render_widget(widget, area);
}
