use crate::domains::entities::todo::Todo;
use crate::domains::repositories::todo_repository::TodoRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct PostgreSQLTodoRepository {
    pub db: sqlx::PgPool,
}

#[async_trait]
impl TodoRepository for PostgreSQLTodoRepository {
    async fn get_all_todos(&self) -> sqlx::Result<Vec<Todo>> {
        sqlx::query_as!(
            Todo,
            "
SELECT *
FROM todos
            "
        )
        .fetch_all(&self.db)
        .await
    }

    async fn create_todo(&self, todo: Todo) -> sqlx::Result<Todo> {
        sqlx::query_as!(
            Todo,
            "
INSERT INTO todos (body, complete, created_at, updated_at)
VALUES ($1, $2, $3, $4)
            ",
            todo.body,
            todo.complete,
            todo.created_at,
            todo.updated_at,
        )
        .execute(&self.db)
        .await?;
        Ok(todo)
    }

    async fn toggle_complete(
        &self,
        id: i32,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> sqlx::Result<bool> {
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
    ) -> sqlx::Result<bool> {
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

    async fn delete_todo(&self, id: i32) -> sqlx::Result<bool> {
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
}
