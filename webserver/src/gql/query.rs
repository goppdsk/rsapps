use crate::auth::create_jwt;
use crate::domains::entities::todo::Todo;
use crate::domains::entities::user::User;
use crate::gql::GraphQLContext;
use juniper::{FieldResult, IntoFieldError};

pub struct QueryRoot;

#[graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    #[graphql(name = "apiVersion")]
    fn api_version() -> &str {
        "0.1.0"
    }

    #[graphql(description = "Get all Users")]
    async fn users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        match context.state.user_service.clone().get_all_users().await {
            Ok(users) => Ok(users),
            Err(err) => Err(err.into_field_error()),
        }
    }

    #[graphql(name = "login", description = "User login")]
    async fn login(
        context: &GraphQLContext,
        username: String,
        password: String,
    ) -> FieldResult<String> {
        let user = match context
            .state
            .user_service
            .clone()
            .get_user_by_username(username, password)
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
        context: &GraphQLContext,
        email: String,
        password: String,
    ) -> FieldResult<String> {
        let user = match context
            .state
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
    async fn todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        match context
            .state
            .todo_service
            .clone()
            .get_all_todos(context.user_id)
            .await
        {
            Ok(todos) => Ok(todos),
            Err(err) => Err(err.into_field_error()),
        }
    }
}
