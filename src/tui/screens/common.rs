use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::Browser;

/// Dibuja el marco con borde y `title` compartido, y devuelve el área interior
/// donde cada pantalla coloca su propio contenido.
pub fn screen_frame(frame: &mut Frame, title: &str) -> Rect {
    let area = frame.area();

    let outer = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(outer, area);

    area.inner(Margin {
        vertical: 2,
        horizontal: 4,
    })
}

/// Renderiza una pantalla genérica de marcador de posición: título,
/// descripción y una ayuda para volver.
///
/// Las pantallas de features la reutilizan mientras su interfaz real no está
/// implementada.
pub fn render_placeholder(frame: &mut Frame, title: &str, description: &str) {
    let inner = screen_frame(frame, title);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let heading = Paragraph::new(Line::from(title))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();

    let body = Paragraph::new(Line::from(description))
        .style(Style::default().fg(Color::Gray))
        .centered();

    let footer = Paragraph::new(Line::from("Esc / Q  Volver"))
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(heading, chunks[0]);
    frame.render_widget(body, chunks[1]);
    frame.render_widget(footer, chunks[2]);
}

/// Renderiza el navegador de archivos (ruta actual + listado) en `area`.
///
/// Aplica un desplazamiento manual para mantener visible la entrada resaltada.
pub fn render_browser(frame: &mut Frame, area: Rect, browser: &Browser) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // ruta actual
            Constraint::Length(1), // separador
            Constraint::Min(1),    // listado
        ])
        .split(area);

    let path = Paragraph::new(Line::from(vec![
        Span::styled("Carpeta: ", Style::default().fg(Color::Gray)),
        Span::styled(
            browser.current_dir.display().to_string(),
            Style::default().fg(Color::Cyan),
        ),
    ]));
    frame.render_widget(path, chunks[0]);

    let list_area = chunks[2];
    let height = list_area.height as usize;

    let start = if browser.selected >= height {
        browser.selected + 1 - height
    } else {
        0
    };

    let lines: Vec<Line> = browser
        .entries
        .iter()
        .enumerate()
        .skip(start)
        .take(height)
        .map(|(index, entry)| {
            let selected = index == browser.selected;
            let marker = if selected { "▶ " } else { "  " };

            let label = if entry.is_dir && !entry.is_parent {
                format!("{}/", entry.name)
            } else {
                entry.name.clone()
            };

            let color = if selected {
                Color::Yellow
            } else if entry.is_dir {
                Color::Cyan
            } else {
                Color::White
            };

            let mut style = Style::default().fg(color);
            if selected {
                style = style.add_modifier(Modifier::BOLD);
            }

            Line::from(Span::styled(format!("{marker}{label}"), style))
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), list_area);
}

/// Párrafo centrado "⏳ Procesando…" para acciones bloqueantes en curso.
pub fn processing() -> Paragraph<'static> {
    Paragraph::new(Line::from(Span::styled(
        "⏳ Procesando…",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )))
    .centered()
}

/// Formatea un número de bytes como B / KB / MB.
pub fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    let b = bytes as f64;

    if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}
