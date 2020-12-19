use bcrypt::verify;

#[derive(Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    fn valid_password(&self, password: String) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}
