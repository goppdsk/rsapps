use crate::domains::entities::todo::Todo;
use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait TodoRepository: DynClone {
    async fn get_all_todos(&self) -> anyhow::Result<Vec<Todo>>;

    async fn create_todo(&self, todo: Todo) -> anyhow::Result<Todo>;

    async fn update_todo(&self, todo: Todo) -> anyhow::Result<Todo>;

    async fn toggle_complete(
        &self,
        id: i32,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<bool>;

    async fn toggle_all_complete(
        &self,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<bool>;

    async fn delete_todo(&self, id: i32) -> anyhow::Result<bool>;

    async fn delete_completed_todo(&self) -> anyhow::Result<bool>;
}

dyn_clone::clone_trait_object!(TodoRepository);
