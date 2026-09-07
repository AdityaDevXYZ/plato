use crate::error::ApplicationError;
use identity::{
    user::User,
    repository::UserRepository,
};
use types::Email;
use std::sync::Arc;

pub struct CreateUserCommand {
    pub email: String,
    pub raw_password: Option<String>,
}

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn create_user(&self, command: CreateUserCommand) -> Result<User, ApplicationError> {
        let email = Email::new(command.email)
            .map_err(|e| ApplicationError::Domain(e.to_string()))?;

        let pass = command.raw_password.as_deref().unwrap_or("PlatoSecurePass123!");
        let user = User::new(email, pass)
            .map_err(|e| ApplicationError::Domain(format!("{:?}", e)))?;

        self.user_repo
            .save(&user)
            .await.map_err(|e| ApplicationError::Repository(e.to_string()))?;

        Ok(user)
    }
}
