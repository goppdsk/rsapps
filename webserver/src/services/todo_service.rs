use crate::domains::entities::todo::Todo;
use crate::domains::errors::{ApplicationError, ErrorCode};
use crate::domains::repositories::todo_repository::TodoRepository;
use crate::domains::ApplicationResult;
use crate::infrastructures::di_container::DIContainer;

#[derive(Clone)]
pub struct TodoService {
    pub todo_repository: Box<dyn TodoRepository + Send + Sync>,
}

impl TodoService {
    pub fn new(di_container: &dyn DIContainer) -> Self {
        Self {
            todo_repository: di_container.todo_repository(),
        }
    }
    pub async fn get_all_todos(&self) -> ApplicationResult<Vec<Todo>> {
        match self.todo_repository.get_all_todos().await {
            Ok(todos) => Ok(todos),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to fetch todos, error: {:}", err),
            }),
        }
    }

    pub async fn create_todo(&self, todo: Todo) -> ApplicationResult<Todo> {
        match self.todo_repository.create_todo(todo).await {
            Ok(created) => Ok(created),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to fetch todos, error: {:}", err),
            }),
        }
    }

    pub async fn toggle_complete(&self, id: i32) -> ApplicationResult<bool> {
        let now = chrono::Utc::now();
        match self.todo_repository.toggle_complete(id, now).await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to complete todo(id: {}), error: {:}", id, err),
            }),
        }
    }

    pub async fn toggle_all_complete(&self) -> ApplicationResult<bool> {
        let now = chrono::Utc::now();
        match self.todo_repository.toggle_all_complete(now).await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to complete all todos, error: {:}", err),
            }),
        }
    }

    pub async fn delete_todo(&self, id: i32) -> ApplicationResult<bool> {
        match self.todo_repository.delete_todo(id).await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to complete all todos, error: {:}", err),
            }),
        }
    }

    pub async fn clear_completed_todo(&self) -> ApplicationResult<bool> {
        match self.todo_repository.delete_completed_todo().await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to delete all completed todos, error: {:}", err),
            }),
        }
    }
}
