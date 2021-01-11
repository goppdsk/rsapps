use crate::domains::entities::todo::Todo;
use crate::domains::repositories::todo_repository::TodoRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct PostgreSQLTodoRepository {
    pub db: sqlx::PgPool,
}

#[async_trait]
impl TodoRepository for PostgreSQLTodoRepository {
    async fn get_all_todos(&self, user_id: i32) -> anyhow::Result<Vec<Todo>> {
        Ok(sqlx::query_as!(
            Todo,
            "
SELECT *
FROM todos
WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&self.db)
        .await?)
    }

    async fn get_todo_by_id(&self, id: i32) -> anyhow::Result<Option<Todo>> {
        Ok(sqlx::query_as!(
            Todo,
            "
SELECT *
FROM todos
WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.db)
        .await?)
    }

    async fn create_todo(&self, todo: Todo) -> anyhow::Result<Todo> {
        Ok(sqlx::query_as!(
            Todo,
            "
INSERT INTO todos (body, complete, created_at, updated_at, user_id)
VALUES ($1, $2, $3, $4, $5)
returning *
            ",
            todo.body,
            todo.complete,
            todo.created_at,
            todo.updated_at,
            todo.user_id,
        )
        .fetch_one(&self.db)
        .await?)
    }

    async fn update_todo(&self, todo: Todo) -> anyhow::Result<Todo> {
        Ok(sqlx::query_as!(
            Todo,
            "
UPDATE todos
SET body = $1, complete = $2, updated_at = $3
WHERE id = $4
returning *
            ",
            todo.body,
            todo.complete,
            todo.updated_at,
            todo.id,
        )
        .fetch_one(&self.db)
        .await?)
    }

    async fn toggle_complete(
        &self,
        id: i32,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<bool> {
        sqlx::query_as!(
            Todo,
            "
UPDATE todos
SET complete = not complete, updated_at = $1
WHERE id = $2
            ",
            updated_at,
            id
        )
        .execute(&self.db)
        .await?;
        Ok(true)
    }

    async fn toggle_all_complete(
        &self,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<bool> {
        sqlx::query_as!(
            Todo,
            "
UPDATE todos
SET complete = not complete, updated_at = $1
            ",
            updated_at
        )
        .execute(&self.db)
        .await?;
        Ok(true)
    }

    async fn delete_todo(&self, id: i32) -> anyhow::Result<bool> {
        sqlx::query_as!(
            Todo,
            "
DELETE
FROM todos
WHERE id = $1
            ",
            id
        )
        .execute(&self.db)
        .await?;
        Ok(true)
    }

    async fn delete_completed_todo(&self) -> anyhow::Result<bool> {
        sqlx::query_as!(
            Todo,
            "
DELETE
FROM todos
WHERE complete = true
            ",
        )
        .execute(&self.db)
        .await?;
        Ok(true)
    }
}
