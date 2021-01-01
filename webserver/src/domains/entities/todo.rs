#[derive(Clone)]
pub struct Todo {
    pub id: i32,
    pub body: String,
    pub complete: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
