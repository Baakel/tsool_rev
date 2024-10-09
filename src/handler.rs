use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::{App, AppResult};
use crate::models::InputMode;

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    app.input_mode = InputMode::Editing
                }
                _ => {}
            }
        }
        InputMode::Editing => {
            match key_event.code {
                KeyCode::Right => {
                    app.move_cursor_right();
                }
                KeyCode::Left => {
                    app.move_cursor_left();
                }
                KeyCode::Esc => {
                    app.input.clear();
                    app.reset_cursor();
                    app.input_mode = InputMode::Normal
                }
                KeyCode::Enter => app.save_todo().await,
                KeyCode::Char(to_insert) => app.enter_char(to_insert),
                KeyCode::Backspace => app.delete_char(),
                // KeyCode::Char('l') => {
                //     app.populate_todos().await;
                // }
                _ => {}
            }
        }
    }
    Ok(())
}