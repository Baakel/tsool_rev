use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Cell, Row, StatefulWidget, Table, TableState},
};
use sqlx::PgPool;

use crate::{db::getters::get_uncompleted_todos, models::Todo};

#[derive(Debug)]
pub struct TodosTableWidget<'a> {
    pub todos: Vec<Todo>,
    pub todos_table: Table<'a>,
}

impl StatefulWidget for &TodosTableWidget<'_> {
    type State = TableState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        StatefulWidget::render(&self.todos_table, area, buf, state);
    }
}

impl TodosTableWidget<'_> {
    pub fn populate_table(&mut self) {
        // self.todos = get_uncompleted_todos(db).await?;
        let rows = self
            .todos
            .iter()
            .map(|todo| {
                let done_char = match todo.done {
                    None => '󰄱',
                    Some(_) => '󰡖',
                };
                Row::new(vec![
                    Cell::new(todo.id.to_string()),
                    Cell::new(todo.value.clone()),
                    Cell::new(done_char.to_string()),
                ])
            })
            .collect::<Vec<Row>>();
        let widths = Constraint::from_percentages([10, 80, 10]);
        self.todos_table = Table::new(rows, widths)
            .block(Block::bordered().title("Todos"))
            .row_highlight_style(Style::default().reversed())
            .highlight_symbol("󰚌 ");

        // Ok(())
    }

    pub fn new() -> Self {
        Self {
            todos: vec![],
            todos_table: Table::default(),
        }
    }
}
