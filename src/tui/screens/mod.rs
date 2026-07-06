pub mod common;
pub mod convert;
pub mod home;
pub mod inspect;
pub mod metadata;
pub mod optimize;
pub mod settings;

use ratatui::Frame;

use crate::app::{AppState, Screen};

/// Delega el renderizado a la pantalla activa en `state`.
pub fn render(frame: &mut Frame, state: &AppState) {
    match state.screen {
        Screen::Home => home::render(frame, state),
        Screen::Optimize => optimize::render(frame, state),
        Screen::Metadata => metadata::render(frame, state),
        Screen::Convert => convert::render(frame, state),
        Screen::Inspect => inspect::render(frame, state),
        Screen::Settings => settings::render(frame, state),
    }
}
