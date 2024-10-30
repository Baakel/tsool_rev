use crate::db::getters::get_uncompleted_todos;
use crate::db::setters::{mark_todo_done, mark_todo_undone, save_todo};
use crate::models::{Goal, InputMode, Todo};
use crate::widgets::goals::GoalsWidget;
use crate::widgets::todos::TodosTableWidget;
use ratatui::widgets::TableState;
use sqlx::PgPool;

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct App<'a> {
    pub running: bool,
    pub todos: Vec<Todo>,
    pub db: PgPool,
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub todos_state: TableState,
    pub todos_table: TodosTableWidget<'a>,
    pub goal_widget: GoalsWidget<'a>,
    pub errors: String,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             running: true,
//             counter: 0,
//             todos: vec![],
//             goals: vec![],
//             db,
//         }
//     }
// }

impl App<'_> {
    pub async fn new(db: PgPool) -> Self {
        Self {
            running: true,
            todos: vec![],
            db,
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Normal,
            todos_state: TableState::default(),
            todos_table: TodosTableWidget::new(),
            goal_widget: GoalsWidget::new(),
            errors: String::new(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub async fn save_todo(&mut self) {
        let todo = Todo::new(self.input.clone());
        save_todo(&self.db, todo).await.unwrap();
        self.reload_todos().await;
        self.input.clear();
        self.reset_cursor();
        self.todos_state = TableState::default();
    }

    pub fn select_next_todo(&mut self) {
        self.todos_state.select_next();
    }

    pub fn select_prev_todo(&mut self) {
        self.todos_state.select_previous();
    }

    pub async fn toggle_todo(&mut self) {
        let todo_index = match self.todos_state.selected() {
            Some(i) => i,
            None => return,
        };

        let row = self.todos_table.todos.get(todo_index);
        if let Some(todo) = row {
            match todo.done {
                None => mark_todo_done(&self.db, todo.id).await.unwrap(),
                Some(_) => mark_todo_undone(&self.db, todo.id).await.unwrap(),
            }
        }
    }

    pub async fn reload_todos(&mut self) {
        let todos = get_uncompleted_todos(&self.db).await;
        let mut errors = String::new();
        let todos = match todos {
            Ok(t) => t,
            Err(e) => {
                errors = e.to_string();
                vec![]
            }
        };
        self.errors = errors;
        self.todos_table.todos = todos;
    }
    // pub fn render_list(&) {
    //
    // }
}
