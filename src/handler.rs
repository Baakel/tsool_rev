use crate::app::{App, AppResult};
use crate::db::getters::get_uncompleted_todos;
use crate::models::InputMode;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App<'_>) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => app.input_mode = InputMode::Editing,
            KeyCode::Down | KeyCode::Char('j') => {
                app.select_next_todo();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.select_prev_todo();
            }
            KeyCode::Enter => {
                app.todos = match get_uncompleted_todos(&app.db).await {
                    Err(e) => {
                        app.errors = e.to_string();
                        vec![]
                    }
                    Ok(t) => t,
                };
                app.toggle_todo().await;
                if let Err(e) = app.todos_table.populate_table(&app.db).await {
                    app.errors = e.to_string();
                };
            }
            _ => {}
        },
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
