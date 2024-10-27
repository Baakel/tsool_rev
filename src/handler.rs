use crate::app::{App, AppResult};
use crate::db::getters::get_all_todos;
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
                // app.todos_state.select_next();
                // let selected = app.todos_state.selected();
                // app.input = format!("{selected:?}");
                // let next = match selected {
                //     None => None,
                //     Some(i) => Some(i + 1),
                // };
                // app.todos_state.select(next);
                // app.input = format!("{selected:?}");
            }
            KeyCode::Up | KeyCode::Char('k') => {
                // app.select_prev_todo();
                app.todos_state.select_previous();
            }
            KeyCode::Enter => {
                app.mark_done().await;
                app.todos = get_all_todos(&app.db).await;
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
