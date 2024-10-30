use chrono::Utc;
use ratatui::{
    style::Stylize,
    text::Text,
    widgets::{Block, Paragraph, Widget},
};

use crate::models::Goal;

#[derive(Debug)]
pub struct GoalsWidget<'a> {
    pub goal: Goal,
    pub goal_para: Paragraph<'a>,
}

impl Widget for &GoalsWidget<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Widget::render(&self.goal_para, area, buf);
    }
}

impl GoalsWidget<'_> {
    pub fn populate_goal(&mut self) {
        let text = Text::from(self.goal.value.clone()).bold();
        let paragraph =
            Paragraph::new(text).block(Block::bordered().title("Today's goal").yellow());
        self.goal_para = paragraph;
    }

    pub fn new() -> Self {
        Self {
            goal: Goal {
                id: 0,
                value: String::from("Some valid goal!"),
                done: None,
                goal_date: Utc::now(),
            },
            goal_para: Paragraph::default(),
        }
    }
}