use crate::domains::entities::user::User;

#[graphql_object]
#[graphql(description = "A user")]
impl User {
    #[graphql(description = "A user id")]
    fn id(&self) -> i32 {
        self.id
    }

    #[graphql(description = "A user name")]
    fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    #[graphql(description = "A user email")]
    fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    #[graphql(name = "createdAt", description = "A user created time")]
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    #[graphql(name = "updatedAt", description = "A user updated time")]
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}
