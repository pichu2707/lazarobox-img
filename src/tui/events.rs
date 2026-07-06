use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEventKind};
use std::time::Duration;

/// Una pulsación de tecla, sin semántica de aplicación.
///
/// El significado de cada tecla depende de la pantalla activa, por lo que la
/// decisión (navegar, escribir, salir, volver…) la toma el bucle de eventos,
/// no este módulo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEvent {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Backspace,
    Char(char),
    None,
}

/// Lee un evento solo si ya hay uno disponible (no bloquea).
///
/// Se usa mientras se procesa un lote para poder cancelar sin frenar el
/// trabajo: si no hay tecla pendiente, devuelve `None` de inmediato.
pub fn poll_event() -> Result<Option<AppEvent>> {
    if event::poll(Duration::from_millis(0))? {
        Ok(Some(read_event()?))
    } else {
        Ok(None)
    }
}

pub fn read_event() -> Result<AppEvent> {
    match event::read()? {
        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Ok(match key.code {
            KeyCode::Up => AppEvent::Up,
            KeyCode::Down => AppEvent::Down,
            KeyCode::Left => AppEvent::Left,
            KeyCode::Right => AppEvent::Right,
            KeyCode::Enter => AppEvent::Enter,
            KeyCode::Esc => AppEvent::Esc,
            KeyCode::Backspace => AppEvent::Backspace,
            KeyCode::Char(c) => AppEvent::Char(c),
            _ => AppEvent::None,
        }),
        _ => Ok(AppEvent::None),
    }
}
