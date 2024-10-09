use chrono::Utc;
use sqlx::PgPool;
use crate::models::{Goal, Todo};

pub async fn save_todo(db: &PgPool, todo: Todo) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO todos (value, done, created) VALUES ($1, $2, $3)",
        todo.value,
        todo.done,
        Utc::now()
    )
        .execute(db)
        .await?;
    Ok(())
}

pub async fn save_goal(db: &PgPool, goal: Goal) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO goals (value, done, goal_date) VALUES ($1, $2, $3)",
        goal.value,
        goal.done,
        goal.goal_date,
    )
        .execute(db)
        .await?;
    Ok(())
}

pub async fn mark_todo_done(db: &PgPool, todo_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE todos SET done = $1 WHERE id = $2",
        Some(Utc::now()),
        todo_id,
    )
        .execute(db)
        .await?;
    Ok(())
}

pub async fn mark_goal_done(db: &PgPool, goal_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE goals SET done = $1 WHERE id = $2",
        Some(Utc::now()),
        goal_id,
    )
        .execute(db)
        .await?;
    Ok(())
}