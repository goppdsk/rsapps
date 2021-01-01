use crate::auth::create_jwt;
use crate::domains::entities::todo::Todo;
use crate::domains::entities::user::User;
use crate::State;
use juniper::{FieldResult, IntoFieldError};

pub struct QueryRoot;

#[graphql_object(Context = State)]
impl QueryRoot {
    #[graphql(name = "apiVersion")]
    fn api_version() -> &str {
        "0.1.0"
    }

    #[graphql(description = "Get all Users")]
    async fn users(context: &State) -> FieldResult<Vec<User>> {
        match context.user_service.clone().get_all_users().await {
            Ok(users) => Ok(users),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(name = "idLogin", description = "User login with id")]
    async fn login_with_id(context: &State, id: i32, password: String) -> FieldResult<String> {
        let user = match context
            .user_service
            .clone()
            .get_user_by_id(id, password)
            .await
        {
            Ok(user) => user,
            Err(err) => return Err(err.into_field_error()),
        };
        match create_jwt(user.id) {
            Ok(jwt) => Ok(jwt),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(name = "emailLogin", description = "User login with email")]
    async fn login_with_email(
        context: &State,
        email: String,
        password: String,
    ) -> FieldResult<String> {
        let user = match context
            .user_service
            .clone()
            .get_user_by_email(email, password)
            .await
        {
            Ok(user) => user,
            Err(err) => return Err(err.into_field_error()),
        };
        match create_jwt(user.id) {
            Ok(jwt) => Ok(jwt),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(description = "Get all todos")]
    async fn todos(context: &State) -> FieldResult<Vec<Todo>> {
        match context.todo_service.clone().get_all_todos().await {
            Ok(todos) => Ok(todos),
            Err(err) => Err(err.into_field_error()),
        }
    }
}
