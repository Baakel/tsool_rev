use crate::models::{Goal, Todo};
use chrono::{NaiveTime, Utc};
use sqlx::PgPool;

pub async fn get_all_todos(db: &PgPool) -> Vec<Todo> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(db)
        .await;

    todos.unwrap_or_else(|e| {
        println!("Error found while fetching todos: {e:?}");
        vec![]
    })
}

pub async fn get_todays_goal(db: &PgPool) -> Vec<Goal> {
    let goal = sqlx::query_as!(
        Goal,
        "SELECT * FROM goals WHERE goal_date <= $1",
        Utc::now()
    )
    .fetch_all(db)
    .await;

    goal.unwrap_or_else(|error| {
        println!("Error found while fetching today's goal: {error:?}");
        vec![]
    })
}

pub async fn get_uncompleted_todos(db: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    let today = Utc::now()
        .with_time(NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap())
        .unwrap();
    let todos = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE done IS NULL OR done >= $1 ORDER BY id DESC",
        today
    )
    .fetch_all(db)
    .await;

    todos
}
