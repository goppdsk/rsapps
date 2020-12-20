use crate::domains::entities::user::User;
use crate::domains::errors::{ApplicationError, ErrorCode};
use crate::domains::ApplicationResult;
use crate::infrastructures::di_container::DIContainer;

#[derive(Clone)]
pub struct UserService {
    di_container: Box<DIContainer>,
}

impl UserService {
    pub fn new(di_container: Box<DIContainer>) -> Self {
        UserService { di_container }
    }
    pub async fn get_all_users(self) -> ApplicationResult<Vec<User>> {
        match self.di_container.user_repository().get_all_users().await {
            Ok(users) => Ok(users),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to fetch users, error: {:}", err),
            }),
        }
    }

    pub async fn get_user_by_id(self, id: i32, password: String) -> ApplicationResult<User> {
        let user = self.di_container.user_repository().get_user_by_id(id).await;
        self.handle_user(user, password)
    }

    pub async fn get_user_by_email(
        self,
        email: String,
        password: String,
    ) -> ApplicationResult<User> {
        let user = self
            .di_container
            .user_repository()
            .get_user_by_email(email)
            .await;
        self.handle_user(user, password)
    }

    fn handle_user(
        self,
        result: sqlx::Result<Option<User>>,
        password: String,
    ) -> ApplicationResult<User> {
        match result {
            Ok(user) => match user {
                Some(user) => {
                    if !user.valid_password(password) {
                        return Err(ApplicationError {
                            code: ErrorCode::UnAuthenticated,
                            message: "user is not authenticated".to_owned(),
                        });
                    }
                    Ok(user)
                }
                None => Err(ApplicationError {
                    code: ErrorCode::NotFound,
                    message: "user is not registered".to_owned(),
                }),
            },
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: format!("failed to fetch user, error: {:}", err),
            }),
        }
    }
}
