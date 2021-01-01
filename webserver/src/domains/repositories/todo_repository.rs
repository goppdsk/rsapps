use crate::domains::entities::todo::Todo;
use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait TodoRepository: DynClone {
    async fn get_all_todos(&self) -> sqlx::Result<Vec<Todo>>;

    async fn create_todo(&self, todo: Todo) -> sqlx::Result<Todo>;

    async fn toggle_complete(
        &self,
        id: i32,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> sqlx::Result<bool>;

    async fn toggle_all_complete(
        &self,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> sqlx::Result<bool>;

    async fn delete_todo(&self, id: i32) -> sqlx::Result<bool>;
}

dyn_clone::clone_trait_object!(TodoRepository);
