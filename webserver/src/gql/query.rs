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
}
