use bcrypt::verify;

#[derive(Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn valid_password(&self, password: String) -> bool {
        let hash = match self.password_hash.as_ref() {
            Some(hash) => hash,
            None => return false,
        };
        verify(password, &hash).unwrap_or(false)
    }
}
