use crate::models::{Goal, Todo};
use chrono::Utc;
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


pub async fn get_uncompleted_todos(db: &PgPool) -> Vec<Todo> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE done IS NULL"
    )
    .fetch_all(db)
    .await;

    todos.unwrap_or_else(|error| {
        println!("Error while fetching uncompleted todos: {error:?}");
        vec![]
    })
}
