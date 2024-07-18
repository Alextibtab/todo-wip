use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{AppResult, TodoApp};

pub fn handle_key_events(key_event: KeyEvent, app: &mut TodoApp) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.exit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.exit();
            }
        }
        _ => {}
    }
    Ok(())
}
