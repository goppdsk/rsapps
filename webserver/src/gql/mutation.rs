use crate::domains::entities::todo::Todo;
use crate::State;
use juniper::{FieldResult, IntoFieldError};

pub struct MutationRoot;

#[derive(juniper::GraphQLInputObject)]
struct NewTodo {
    body: String,
}

#[graphql_object(Context = State)]
impl MutationRoot {
    #[graphql(description = "Get all todos")]
    async fn create_todo(context: &State, new_todo: NewTodo) -> FieldResult<Todo> {
        let now = chrono::Utc::now();
        let todo = Todo {
            id: 0,
            body: new_todo.body,
            complete: false,
            created_at: now,
            updated_at: now,
        };
        match context.todo_service.clone().create_todo(todo).await {
            Ok(created) => Ok(created),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(name = "toggleComplete", description = "Toggle todo complete")]
    async fn toggle_complete(context: &State, id: i32) -> FieldResult<bool> {
        match context.todo_service.clone().toggle_complete(id).await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(name = "toggleAllComplete", description = "Toggle all todo complete")]
    async fn toggle_all_complete(context: &State) -> FieldResult<bool> {
        match context.todo_service.clone().toggle_all_complete().await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(name = "deleteTodo", description = "Delete todo")]
    async fn delete_todo(context: &State, id: i32) -> FieldResult<bool> {
        match context.todo_service.clone().delete_todo(id).await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(err.into_field_error()),
        }
    }
}
