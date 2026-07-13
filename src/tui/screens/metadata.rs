use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::{AppState, MetaField};
use crate::metadata::MetadataField;
use crate::tui::screens::common;

pub fn render(frame: &mut Frame, state: &AppState) {
    let inner = common::screen_frame(frame, "Metadatos");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // título
            Constraint::Length(1), // separador
            Constraint::Min(1),    // navegador / resultado / formulario
            Constraint::Length(1), // pie
        ])
        .split(inner);

    let heading = Paragraph::new(Line::from("Metadatos"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();
    frame.render_widget(heading, chunks[0]);

    let view = &state.metadata_view;

    if view.viewing {
        if view.pending.is_some() {
            frame.render_widget(common::processing(), chunks[2]);
            frame.render_widget(footer("Procesando…"), chunks[3]);
        } else if view.editing {
            frame.render_widget(edit_body(state), chunks[2]);
            frame.render_widget(
                footer("↑↓ Campo   ←→/Enter Alternar   Enter Siguiente/Guardar   Esc Cancelar"),
                chunks[3],
            );
        } else {
            frame.render_widget(result_body(state), chunks[2]);
            frame.render_widget(footer("e Editar      Enter/Esc Volver"), chunks[3]);
        }
    } else {
        common::render_browser(frame, chunks[2], &state.browser);
        frame.render_widget(
            footer("↑↓ Mover   Enter Abrir/Leer   Backspace Subir   Esc Volver"),
            chunks[3],
        );
    }
}

fn footer(text: &str) -> Paragraph<'_> {
    Paragraph::new(Line::from(text)).style(Style::default().fg(Color::DarkGray))
}

/// Metadatos leídos de la última imagen (con aviso de guardado o error).
fn result_body(state: &AppState) -> Paragraph<'static> {
    let view = &state.metadata_view;

    if let Some(error) = &view.error {
        return Paragraph::new(Line::from(Span::styled(
            format!("✖ {error}"),
            Style::default().fg(Color::Red),
        )));
    }

    let Some(meta) = &view.result else {
        return Paragraph::new(Line::from(""));
    };

    let mut lines = Vec::new();

    if let Some(saved) = &view.saved {
        lines.push(Line::from(Span::styled(
            saved.clone(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    lines.extend([
        section("WEB"),
        field_line("Descripción", &meta.web.description),
        field_line("Texto alt", &meta.web.alt_text),
        Line::from(""),
        section("DERECHOS"),
        field_line("Autor", &meta.rights.author),
        field_line("Copyright", &meta.rights.copyright),
        Line::from(""),
        section("IA"),
        bool_line("Detectada", meta.ai.detected),
        field_line("Software", &meta.ai.software),
        Line::from(""),
        section("PRIVACIDAD"),
        bool_line("Datos GPS", meta.privacy.has_gps),
    ]);

    if let (Some(lat), Some(lon)) = (meta.seo.latitude.value, meta.seo.longitude.value) {
        lines.push(coord_line(lat, lon));
    }

    lines.extend([
        Line::from(""),
        Line::from(Span::styled(
            "Pulsa e para editar alt text, GPS y limpieza IA.",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            "Escritura soportada: JPEG y WebP.",
            Style::default().fg(Color::DarkGray),
        )),
    ]);

    Paragraph::new(lines)
}

/// Formulario de edición de metadatos.
fn edit_body(state: &AppState) -> Paragraph<'static> {
    let view = &state.metadata_view;
    let f = view.focus;

    let mut lines = vec![
        Line::from(Span::styled(
            "Editar metadatos (se guarda en el archivo)",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        text_field("Alt text", &view.alt_input, f == MetaField::AltText),
        text_field("Latitud GPS", &view.lat_input, f == MetaField::Lat),
        text_field("Longitud GPS", &view.lon_input, f == MetaField::Lon),
        toggle_field("Quitar GPS", view.remove_gps, f == MetaField::RemoveGps),
        toggle_field("Quitar IA", view.remove_ai, f == MetaField::RemoveAi),
        Line::from(Span::styled(
            "GPS: rellena latitud y longitud para cambiar; Quitar GPS elimina ubicación.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        save_line(f == MetaField::Save),
    ];

    if let Some(error) = &view.error {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("✖ {error}"),
            Style::default().fg(Color::Red),
        )));
    }

    Paragraph::new(lines)
}

/// Campo de texto del formulario, con cursor cuando está enfocado.
fn text_field(label: &str, value: &str, focused: bool) -> Line<'static> {
    let (marker, color) = marker(focused);
    let shown = if focused {
        format!("{value}▏")
    } else {
        value.to_string()
    };

    Line::from(vec![
        Span::styled(
            format!("{marker}{label:<13}"),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(shown, Style::default().fg(color)),
    ])
}

/// Campo booleano del formulario ("[x]" / "[ ]").
fn toggle_field(label: &str, on: bool, focused: bool) -> Line<'static> {
    let (marker, color) = marker(focused);
    let mark = if on { "[x]" } else { "[ ]" };

    Line::from(vec![
        Span::styled(
            format!("{marker}{label:<13}"),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(mark.to_string(), Style::default().fg(color)),
    ])
}

/// Línea del botón "Guardar".
fn save_line(focused: bool) -> Line<'static> {
    let (marker, color) = marker(focused);
    Line::from(Span::styled(
        format!("{marker}[ Guardar ]"),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}

/// Marcador "▶ " + color según foco.
fn marker(focused: bool) -> (&'static str, Color) {
    if focused {
        ("▶ ", Color::Yellow)
    } else {
        ("  ", Color::White)
    }
}

/// Cabecera de sección en cian y negrita.
fn section(title: &str) -> Line<'static> {
    Line::from(Span::styled(
        title.to_string(),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

/// Campo de texto: "✓ valor" si existe, "✗" si falta.
fn field_line(label: &str, field: &MetadataField<String>) -> Line<'static> {
    let label_span = Span::styled(format!("  {label:<12}"), Style::default().fg(Color::Gray));

    match &field.value {
        Some(value) => Line::from(vec![
            label_span,
            Span::styled(format!("✓ {value}"), Style::default().fg(Color::White)),
        ]),
        None => Line::from(vec![
            label_span,
            Span::styled("✗", Style::default().fg(Color::DarkGray)),
        ]),
    }
}

/// Línea con las coordenadas GPS en grados decimales.
fn coord_line(lat: f64, lon: f64) -> Line<'static> {
    Line::from(vec![
        Span::styled("  Coordenadas ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{lat:.4}, {lon:.4}"),
            Style::default().fg(Color::White),
        ),
    ])
}

/// Indicador booleano: "✓ sí" / "✗ no".
fn bool_line(label: &str, present: bool) -> Line<'static> {
    let label_span = Span::styled(format!("  {label:<12}"), Style::default().fg(Color::Gray));

    let (text, color) = if present {
        ("✓ sí", Color::White)
    } else {
        ("✗ no", Color::DarkGray)
    };

    Line::from(vec![
        label_span,
        Span::styled(text.to_string(), Style::default().fg(color)),
    ])
}
