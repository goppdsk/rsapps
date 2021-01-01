use crate::domains::entities::todo::Todo;

#[graphql_object]
#[graphql(description = "A todo")]
impl Todo {
    #[graphql(description = "A todo id")]
    fn id(&self) -> i32 {
        self.id
    }

    #[graphql(description = "A todo body")]
    fn body(&self) -> &str {
        self.body.as_str()
    }

    #[graphql(description = "A todo complete")]
    fn complete(&self) -> bool {
        self.complete
    }

    #[graphql(name = "createdAt", description = "A todo created time")]
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    #[graphql(name = "updatedAt", description = "A todo updated time")]
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}
