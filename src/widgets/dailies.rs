use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Cell, Row, StatefulWidget, Table, TableState},
};

use crate::models::Daily;

#[derive(Debug)]
pub struct DailiesWidget<'a> {
    pub dailies: Vec<Daily>,
    pub dailies_table: Table<'a>,
}

impl StatefulWidget for &DailiesWidget<'_> {
    type State = TableState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        StatefulWidget::render(&self.dailies_table, area, buf, state);
    }
}

impl DailiesWidget<'_> {
    pub fn populate_dailies(&mut self) {
        let rows = self
            .dailies
            .iter()
            .map(|daily| {
                let mut done_style = Style::default().blue();
                if daily.done {
                    done_style = done_style.reversed();
                }
                let done_char = if daily.done { '' } else { '' };
                Row::new(vec![
                    Cell::new(daily.id.to_string()),
                    Cell::new(daily.value.to_string()),
                    Cell::new(done_char.to_string()),
                    Cell::new(daily.streak.to_string()),
                ])
                .style(done_style)
            })
            .collect::<Vec<_>>();
        let widths = Constraint::from_percentages([5, 79, 8, 8]);
        self.dailies_table = Table::new(rows, widths)
            .block(Block::bordered().title("Dailies"))
            .style(Style::default().blue())
            .header(
                Row::new(vec![
                    Cell::from("Id"),
                    Cell::from("Value"),
                    Cell::from("Status"),
                    Cell::from("Streak"),
                ])
                .style(Style::default().underlined()),
            )
            .row_highlight_style(Style::default().blue().reversed())
            .highlight_symbol(" ");
    }

    pub fn new() -> Self {
        let mut daily = Daily::new("some done daily".to_string());
        daily.done = true;
        Self {
            dailies: vec![Daily::new("some daily".to_string()), daily],
            dailies_table: Table::default(),
        }
    }
}
