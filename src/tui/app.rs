use anyhow::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::Stdout;

use crate::app::{AppController, OptimizeStep, Screen};
use crate::tui::events::{self, AppEvent};
use crate::tui::screens;

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    controller: &mut AppController,
) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            screens::render(frame, &controller.state);
        })?;

        // Trabajo diferido: ya se dibujó el aviso "Procesando…" / la barra,
        // así que ahora ejecutamos la operación bloqueante sin leer eventos.
        match controller.state.screen {
            Screen::Inspect if controller.state.inspect.pending.is_some() => {
                controller.inspect_run_pending();
                continue;
            }
            Screen::Convert if controller.state.convert.pending.is_some() => {
                controller.convert_run_pending();
                continue;
            }
            Screen::Metadata if controller.state.metadata_view.pending.is_some() => {
                controller.metadata_run_pending();
                continue;
            }
            Screen::Settings
                if controller.state.update.status == crate::update::UpdateStatus::Checking =>
            {
                controller.settings_run_update_check();
                continue;
            }
            Screen::Settings
                if controller.state.update.status == crate::update::UpdateStatus::Updating =>
            {
                controller.settings_run_homebrew_update();
                continue;
            }
            Screen::Optimize if controller.state.optimize.step == OptimizeStep::Running => {
                // Una imagen por fotograma; Esc cancela conservando lo hecho.
                if matches!(events::poll_event()?, Some(AppEvent::Esc)) {
                    controller.optimize_cancel();
                } else {
                    controller.optimize_tick();
                }
                continue;
            }
            _ => {}
        }

        let event = events::read_event()?;

        match controller.state.screen {
            Screen::Home => match event {
                AppEvent::Up => controller.select_previous(),
                AppEvent::Down => controller.select_next(),
                AppEvent::Enter => match controller.current_item().target() {
                    Some(screen) => controller.go_to(screen),
                    None => break, // "Salir" cierra la aplicación.
                },
                AppEvent::Esc | AppEvent::Char('q') | AppEvent::Char('Q') => break,
                _ => {}
            },

            Screen::Optimize => handle_optimize(controller, event),
            Screen::Inspect => handle_inspect(controller, event),
            Screen::Convert => handle_convert(controller, event),
            Screen::Metadata => handle_metadata(controller, event),
            Screen::Settings => handle_settings(controller, event),
        }
    }

    Ok(())
}

/// Ajustes: muestra versión actual y permite comprobar actualizaciones.
fn handle_settings(controller: &mut AppController, event: AppEvent) {
    if controller.state.update.status == crate::update::UpdateStatus::Confirming {
        match event {
            AppEvent::Enter => controller.settings_start_homebrew_update(),
            AppEvent::Esc => controller.settings_cancel_homebrew_update(),
            _ => {}
        }
        return;
    }

    match event {
        AppEvent::Esc | AppEvent::Char('q') | AppEvent::Char('Q') => controller.go_to(Screen::Home),
        AppEvent::Char('c') | AppEvent::Char('C') => controller.settings_check_updates(),
        AppEvent::Char('u') | AppEvent::Char('U') => controller.settings_confirm_homebrew_update(),
        _ => {}
    }
}

/// Inspección: navegar hasta un archivo y elegirlo, o ver el resultado.
fn handle_inspect(controller: &mut AppController, event: AppEvent) {
    if controller.state.inspect.viewing {
        match event {
            AppEvent::Esc | AppEvent::Enter => controller.inspect_back(),
            _ => {}
        }
        return;
    }

    match event {
        AppEvent::Esc => controller.go_to(Screen::Home),
        AppEvent::Up => controller.browser_up(),
        AppEvent::Down => controller.browser_down(),
        AppEvent::Backspace => controller.browser_parent(),
        AppEvent::Enter => controller.inspect_activate(),
        _ => {}
    }
}

/// Conversión: navegar y elegir archivo; ←→ cambian el formato de destino.
fn handle_convert(controller: &mut AppController, event: AppEvent) {
    if controller.state.convert.viewing {
        match event {
            AppEvent::Esc | AppEvent::Enter => controller.convert_back(),
            _ => {}
        }
        return;
    }

    match event {
        AppEvent::Esc => controller.go_to(Screen::Home),
        AppEvent::Up => controller.browser_up(),
        AppEvent::Down => controller.browser_down(),
        AppEvent::Left => controller.convert_previous_format(),
        AppEvent::Right => controller.convert_next_format(),
        AppEvent::Backspace => controller.browser_parent(),
        AppEvent::Enter => controller.convert_activate(),
        _ => {}
    }
}

/// Metadatos: navegar → leer → ver → editar y guardar en disco.
fn handle_metadata(controller: &mut AppController, event: AppEvent) {
    let view = &controller.state.metadata_view;

    if view.viewing {
        if view.editing {
            handle_metadata_edit(controller, event);
        } else {
            // Vista de solo lectura del resultado.
            match event {
                AppEvent::Char('e') | AppEvent::Char('E') => controller.metadata_start_edit(),
                AppEvent::Esc | AppEvent::Enter => controller.metadata_back(),
                _ => {}
            }
        }
        return;
    }

    match event {
        AppEvent::Esc => controller.go_to(Screen::Home),
        AppEvent::Up => controller.browser_up(),
        AppEvent::Down => controller.browser_down(),
        AppEvent::Backspace => controller.browser_parent(),
        AppEvent::Enter => controller.metadata_activate(),
        _ => {}
    }
}

/// Eventos del formulario de edición de metadatos.
fn handle_metadata_edit(controller: &mut AppController, event: AppEvent) {
    use crate::app::MetaField;

    let focus = controller.state.metadata_view.focus;

    match event {
        AppEvent::Esc => controller.metadata_cancel_edit(),
        AppEvent::Up => controller.metadata_focus_previous(),
        AppEvent::Down => controller.metadata_focus_next(),
        AppEvent::Left | AppEvent::Right => controller.metadata_toggle(),
        AppEvent::Backspace => controller.metadata_edit_backspace(),
        AppEvent::Char(c) => controller.metadata_edit_char(c),
        AppEvent::Enter => match focus {
            MetaField::Save => controller.metadata_save(),
            MetaField::RemoveGps | MetaField::RemoveAi => controller.metadata_toggle(),
            _ => controller.metadata_focus_next(),
        },
        AppEvent::None => {}
    }
}

/// Optimización según el paso: navegar entrada, configurar o ver resultados.
///
/// El wizard nunca cierra la aplicación: Esc retrocede un paso y, desde el
/// primero, vuelve al Home.
fn handle_optimize(controller: &mut AppController, event: AppEvent) {
    match controller.state.optimize.step {
        OptimizeStep::Input => match event {
            AppEvent::Esc => controller.go_to(Screen::Home),
            AppEvent::Up => controller.browser_up(),
            AppEvent::Down => controller.browser_down(),
            AppEvent::Backspace => controller.browser_parent(),
            AppEvent::Enter => controller.optimize_activate(),
            AppEvent::Char(' ') => controller.optimize_use_current_dir(),
            _ => {}
        },

        OptimizeStep::Config => match event {
            AppEvent::Esc => controller.optimize_back_to_input(),
            AppEvent::Up => controller.optimize_focus_previous(),
            AppEvent::Down => controller.optimize_focus_next(),
            AppEvent::Left => controller.optimize_adjust(-1),
            AppEvent::Right => controller.optimize_adjust(1),
            AppEvent::Enter => controller.optimize_start_run(),
            AppEvent::Backspace => controller.optimize_config_backspace(),
            AppEvent::Char(c) => controller.optimize_config_char(c),
            _ => {}
        },

        // El procesamiento se conduce desde el bucle principal (tick).
        OptimizeStep::Running => {}

        OptimizeStep::Results => match event {
            AppEvent::Esc => controller.go_to(Screen::Home),
            AppEvent::Enter => controller.optimize_back_to_config(),
            _ => {}
        },
    }
}
