use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Cell, Row, StatefulWidget, Table, TableState},
};

use crate::models::Todo;

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
                let mut done_style = Style::default();
                let done_char = match todo.done {
                    None => '󰄱',
                    Some(_) => {
                        done_style = done_style.dim();
                        '󰡖'
                    }
                };
                Row::new(vec![
                    Cell::new(todo.id.to_string()),
                    Cell::new(todo.value.clone()).style(if todo.done.is_some() {
                        done_style.crossed_out()
                    } else {
                        done_style
                    }),
                    Cell::new(done_char.to_string()),
                ])
                .style(done_style)
            })
            .collect::<Vec<Row>>();
        let widths = Constraint::from_percentages([5, 85, 10]);
        self.todos_table = Table::new(rows, widths)
            .block(Block::bordered().title("Todos"))
            .header(
                Row::new(vec![
                    Cell::from("Id"),
                    Cell::from("Value"),
                    Cell::from("Done"),
                ])
                .style(Style::default().underlined()),
            )
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
