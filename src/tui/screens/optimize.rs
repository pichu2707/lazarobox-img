use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Gauge, Paragraph},
};

use crate::app::{AppState, OptimizeField, OptimizeStep};
use crate::tui::screens::common;

pub fn render(frame: &mut Frame, state: &AppState) {
    let inner = common::screen_frame(frame, "Optimizar imágenes");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // título
            Constraint::Length(1), // paso
            Constraint::Length(1), // separador
            Constraint::Min(1),    // cuerpo del paso
            Constraint::Length(1), // pie
        ])
        .split(inner);

    let heading = Paragraph::new(Line::from("Optimizar imágenes"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .centered();

    frame.render_widget(heading, chunks[0]);
    frame.render_widget(step_label(state), chunks[1]);

    match state.optimize.step {
        OptimizeStep::Input => {
            common::render_browser(frame, chunks[3], &state.browser);
            frame.render_widget(
                footer("↑↓ Mover   Enter Abrir/Elegir   Espacio Usar carpeta   Backspace Subir   Esc Volver"),
                chunks[4],
            );
        }
        OptimizeStep::Config => {
            frame.render_widget(config_body(state), chunks[3]);
            frame.render_widget(
                footer("↑↓ Campo   ←→ Ajustar   0-9 Anchura/Alto   Enter Optimizar   Esc Atrás"),
                chunks[4],
            );
        }
        OptimizeStep::Running => {
            render_running(frame, chunks[3], state);
            frame.render_widget(footer("Esc  Cancelar"), chunks[4]);
        }
        OptimizeStep::Results => {
            frame.render_widget(results_body(state), chunks[3]);
            frame.render_widget(footer("Enter  Ajustar y repetir      Esc  Inicio"), chunks[4]);
        }
    }
}

fn footer(text: &str) -> Paragraph<'_> {
    Paragraph::new(Line::from(text)).style(Style::default().fg(Color::DarkGray))
}

/// Indicador "Paso N/3 · descripción".
fn step_label(state: &AppState) -> Paragraph<'static> {
    let text = match state.optimize.step {
        OptimizeStep::Input => "Paso 1/3 · Seleccionar entrada",
        OptimizeStep::Config => "Paso 2/3 · Configurar",
        OptimizeStep::Running => "Optimizando…",
        OptimizeStep::Results => "Paso 3/3 · Resultados",
    };

    Paragraph::new(Line::from(text)).style(Style::default().fg(Color::Gray))
}

/// Cuerpo del paso de configuración: entrada, anchura, alto, calidad, formato.
fn config_body(state: &AppState) -> Paragraph<'static> {
    let opt = &state.optimize;

    Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Entrada    ", Style::default().fg(Color::Gray)),
            Span::styled(opt.input.clone(), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        field_line(
            "Anchura",
            dimension_value(&opt.width, opt.focus == OptimizeField::Width),
            opt.focus == OptimizeField::Width,
        ),
        field_line(
            "Alto",
            dimension_value(&opt.height, opt.focus == OptimizeField::Height),
            opt.focus == OptimizeField::Height,
        ),
        field_line(
            "Calidad",
            format!("{}", opt.quality),
            opt.focus == OptimizeField::Quality,
        ),
        field_line(
            "Formato",
            opt.format.extension().to_uppercase(),
            opt.focus == OptimizeField::Format,
        ),
    ])
}

/// Barra de progreso + archivo en curso durante la optimización.
fn render_running(frame: &mut Frame, area: ratatui::layout::Rect, state: &AppState) {
    let progress = &state.optimize.progress;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // archivo en curso
            Constraint::Length(1), // separador
            Constraint::Length(1), // barra
        ])
        .split(area);

    let current = Paragraph::new(Line::from(vec![
        Span::styled("Procesando: ", Style::default().fg(Color::Gray)),
        Span::styled(progress.current_file.clone(), Style::default().fg(Color::White)),
    ]));
    frame.render_widget(current, rows[0]);

    let ratio = if progress.total == 0 {
        0.0
    } else {
        progress.done as f64 / progress.total as f64
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .ratio(ratio)
        .label(format!("{} / {}", progress.done, progress.total));
    frame.render_widget(gauge, rows[2]);
}

/// Cuerpo del paso de resultados: error o resumen.
fn results_body(state: &AppState) -> Paragraph<'static> {
    if let Some(error) = &state.optimize.error {
        return Paragraph::new(Line::from(Span::styled(
            format!("✖ {error}"),
            Style::default().fg(Color::Red),
        )));
    }

    if let Some(summary) = &state.optimize.results {
        let saved = savings_percent(summary.original_total, summary.optimized_total);

        let header = if state.optimize.cancelled {
            Span::styled(
                "⚠ Cancelado (resultados parciales)",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                "✔ Optimización completada",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
        };

        return Paragraph::new(vec![
            Line::from(header),
            Line::from(""),
            summary_line("Imágenes", format!("{}", summary.count)),
            summary_line("Original", common::human_size(summary.original_total)),
            summary_line("Optimizado", common::human_size(summary.optimized_total)),
            summary_line("Ahorro", format!("{saved:.1} %")),
            summary_line("Salida", summary.output_dir.display().to_string()),
        ]);
    }

    Paragraph::new(Line::from(Span::styled(
        "Sin resultados todavía.",
        Style::default().fg(Color::DarkGray),
    )))
}

/// Muestra una dimensión: "auto" si está vacía, con cursor si está enfocada.
fn dimension_value(raw: &str, focused: bool) -> String {
    match (raw.is_empty(), focused) {
        (true, true) => "▏  (auto)".to_string(),
        (true, false) => "auto".to_string(),
        (false, true) => format!("{raw}▏ px"),
        (false, false) => format!("{raw} px"),
    }
}

/// Línea de campo de configuración, con marcador "▶" cuando está enfocado.
fn field_line(label: &str, value: String, focused: bool) -> Line<'static> {
    let (marker, value_color) = if focused {
        ("▶ ", Color::Yellow)
    } else {
        ("  ", Color::White)
    };

    Line::from(vec![
        Span::styled(
            format!("{marker}{label:<10}"),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(
            value,
            Style::default()
                .fg(value_color)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

/// Línea "etiqueta      valor" del resumen.
fn summary_line(label: &str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<12}"), Style::default().fg(Color::Gray)),
        Span::styled(value, Style::default().fg(Color::White)),
    ])
}

/// Porcentaje ahorrado; 0 si no hay tamaño original.
fn savings_percent(original: u64, optimized: u64) -> f64 {
    if original == 0 {
        return 0.0;
    }

    (1.0 - optimized as f64 / original as f64) * 100.0
}
