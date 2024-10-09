use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: i64,
    pub value: String,
    pub done: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
}

impl Todo {
    pub fn new(value: String) -> Self {
        Self {
            id: 0,
            value,
            done: None,
            created: Utc::now(),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct Goal {
    pub id: i64,
    pub value: String,
    pub done: Option<DateTime<Utc>>,
    pub goal_date: DateTime<Utc>,
}

#[derive(Debug)]
pub enum InputMode {
    Editing,
    Normal,
}