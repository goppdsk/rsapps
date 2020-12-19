use crate::domains::errors::{ApplicationError, ErrorCode};
use crate::domains::models::user::User;
use juniper::{FieldError, IntoFieldError, ScalarValue};
use std::convert::AsRef;

#[graphql_object]
#[graphql(description = "A user")]
impl User {
    #[graphql(description = "A user id")]
    fn id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    #[graphql(description = "A user name")]
    fn username(&self) -> &str {
        &self.username
    }

    #[graphql(description = "A user password")]
    fn password(&self) -> &str {
        &self.password
    }

    #[graphql(description = "A user email")]
    fn email(&self) -> &str {
        &self.email
    }
}

impl<S: ScalarValue> IntoFieldError<S> for ApplicationError {
    fn into_field_error(self) -> FieldError<S> {
        let code = self.code.as_ref();
        match self.code {
            ErrorCode::SystemError => FieldError::new(
                self.message,
                graphql_value!({
                  "code": code,
                }),
            ),
            _ => FieldError::new(
                "Un exptected error is occured",
                graphql_value!({
                  "code": "UnExptected",
                }),
            ),
        }
    }
}
