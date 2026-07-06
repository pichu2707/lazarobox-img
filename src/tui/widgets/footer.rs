use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};

pub fn render(frame: &mut Frame, area: Rect) {
    let footer = "↑ ↓ Navegar    Enter Seleccionar    Q Salir";

    let widget = Paragraph::new(footer).style(Style::default().fg(Color::DarkGray));

    frame.render_widget(widget, area);
}
