use crate::app::{App, AppResult};
use crate::models::{InputMode, InputType};
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
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.input_type = InputType::Todo;
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                if key_event.modifiers == KeyModifiers::ALT {
                    app.complete_goal().await;
                    app.goal_widget.populate_goal();
                    return Ok(());
                }
                app.input_type = InputType::Goal;
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.select_next_todo();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.select_prev_todo();
            }
            KeyCode::Enter => {
                app.toggle_todo().await;
                app.reload_todos().await;
                app.todos_table.populate_table();
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
                KeyCode::Enter => match app.input_type {
                    InputType::Todo => {
                        app.save_todo().await;
                        app.todos_table.populate_table();
                    }
                    InputType::Goal => {
                        app.save_goal().await;
                        app.get_todays_goal().await;
                        app.goal_widget.populate_goal();
                        app.input_mode = InputMode::Normal;
                        app.input_type = InputType::Todo;
                    }
                    _ => {}
                },
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
