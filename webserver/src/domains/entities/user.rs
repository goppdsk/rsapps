use bcrypt::verify;

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_valid_password() {
        let now = chrono::Utc::now();
        let password = "password";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        let user = User {
            id: 1,
            username: "testuser".to_owned(),
            email: None,
            password_hash: Some(hash),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(true, user.valid_password(password.to_owned()));
    }

    #[test]
    fn test_valid_password_failed() {
        let now = chrono::Utc::now();
        let password = "password";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        let user = User {
            id: 1,
            username: "testuser".to_owned(),
            email: None,
            password_hash: Some(hash),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(false, user.valid_password("invalid".to_owned()));
    }
}
