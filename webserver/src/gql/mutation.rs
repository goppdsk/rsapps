use crate::auth::create_jwt;
use crate::domains::entities::todo::Todo;
use crate::State;
use juniper::{FieldResult, IntoFieldError};

pub struct MutationRoot;

#[derive(juniper::GraphQLInputObject)]
struct NewTodo {
    body: String,
}

#[derive(juniper::GraphQLInputObject)]
struct UpdatedTodo {
    id: i32,
    body: String,
    complete: bool,
}

#[derive(juniper::GraphQLInputObject)]
struct NewUser {
    username: String,
    password: String,
}

#[graphql_object(Context = State)]
impl MutationRoot {
    #[graphql(description = "Create new todo")]
    async fn create_todo(context: &State, new_todo: NewTodo) -> FieldResult<Todo> {
        match context
            .todo_service
            .clone()
            .create_todo(new_todo.body)
            .await
        {
            Ok(created) => Ok(created),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(description = "Update todo")]
    async fn update_todo(context: &State, updated_todo: UpdatedTodo) -> FieldResult<Todo> {
        match context
            .todo_service
            .clone()
            .update_todo(updated_todo.id, updated_todo.body, updated_todo.complete)
            .await
        {
            Ok(updatde) => Ok(updatde),
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

    #[graphql(name = "clearCompletedTodo", description = "Delete all completed todo")]
    async fn clear_completed_todo(context: &State) -> FieldResult<bool> {
        match context.todo_service.clone().clear_completed_todo().await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(name = "signUp", description = "Sign up user")]
    async fn sing_up(context: &State, new_user: NewUser) -> FieldResult<String> {
        let user = match context
            .user_service
            .clone()
            .sign_up(new_user.username, new_user.password)
            .await
        {
            Ok(created) => created,
            Err(err) => return Err(err.into_field_error()),
        };
        match create_jwt(user.id) {
            Ok(jwt) => Ok(jwt),
            Err(err) => Err(err.into_field_error()),
        }
    }
}
