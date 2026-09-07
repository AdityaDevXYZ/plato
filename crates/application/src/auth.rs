use crate::error::ApplicationError;
use identity::repository::UserRepository;
use types::Email;
use std::sync::Arc;
use eventbus::{EventPublisher, EventMessage, Event};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use time::{OffsetDateTime, Duration};
use uuid::Uuid;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct UserLoggedIn { pub user_id: String }
impl Event for UserLoggedIn { fn event_type(&self) -> &'static str { "UserLoggedIn" } fn as_any(&self) -> &dyn Any { self } }

#[derive(Debug, Clone)]
pub struct UserLoggedOut { pub user_id: String }
impl Event for UserLoggedOut { fn event_type(&self) -> &'static str { "UserLoggedOut" } fn as_any(&self) -> &dyn Any { self } }

#[derive(Debug, Clone)]
pub struct AuthenticationFailed { pub email: String, pub reason: String }
impl Event for AuthenticationFailed { fn event_type(&self) -> &'static str { "AuthenticationFailed" } fn as_any(&self) -> &dyn Any { self } }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

#[derive(Debug, Clone)]
pub struct LoginCommand {
    pub email: String,
    pub raw_password: String,
    pub correlation_id: Uuid,
}

pub struct AuthenticationService {
    user_repo: Arc<dyn UserRepository>,
    event_publisher: Arc<dyn EventPublisher>,
    jwt_secret: String,
}

impl AuthenticationService {
    pub fn new(user_repo: Arc<dyn UserRepository>, event_publisher: Arc<dyn EventPublisher>, jwt_secret: String) -> Self {
        Self { user_repo, event_publisher, jwt_secret }
    }

    #[tracing::instrument(skip(self))]
    pub async fn login(&self, command: LoginCommand) -> Result<String, ApplicationError> {
        let email = Email::new(command.email.clone())
            .map_err(|_| ApplicationError::Validation("Invalid email".into()))?;

        let user_opt = self.user_repo.find_by_email(&email)
            .await.map_err(|e| ApplicationError::Repository(e.to_string()))?;

        let user = match user_opt {
            Some(u) => u,
            None => {
                let fail = EventMessage::new(Box::new(AuthenticationFailed { email: command.email, reason: "Not found".into() }), command.correlation_id, "auth");
                let _ = self.event_publisher.publish(vec![fail]);
                return Err(ApplicationError::Validation("Invalid credentials".into()));
            }
        };

        if let Err(_) = user.verify_password(&command.raw_password) {
            let fail = EventMessage::new(Box::new(AuthenticationFailed { email: command.email, reason: "Invalid password".into() }), command.correlation_id, "auth");
            let _ = self.event_publisher.publish(vec![fail]);
            return Err(ApplicationError::Validation("Invalid credentials".into()));
        }

        let expiration = OffsetDateTime::now_utc() + Duration::hours(24);
        let claims = Claims {
            sub: uuid::Uuid::from(user.id()).to_string(),
            role: "Administrator".into(),
            exp: expiration.unix_timestamp(),
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(self.jwt_secret.as_bytes()))
            .map_err(|e| ApplicationError::Domain(format!("JWT Error: {}", e)))?;

        let success = EventMessage::new(Box::new(UserLoggedIn { user_id: claims.sub }), command.correlation_id, "auth");
        let _ = self.event_publisher.publish(vec![success]);

        Ok(token)
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims, ApplicationError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default()
        ).map_err(|_| ApplicationError::Validation("Invalid or expired token".into()))?;
        
        Ok(token_data.claims)
    }

    pub fn refresh_token(&self) -> Result<String, ApplicationError> {
        Err(ApplicationError::Validation("Not implemented".into()))
    }

    pub fn logout(&self, user_id: String, correlation_id: Uuid) -> Result<(), ApplicationError> {
        let event = EventMessage::new(Box::new(UserLoggedOut { user_id }), correlation_id, "auth");
        let _ = self.event_publisher.publish(vec![event]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_validation_and_expiration() {
        let secret = "supersecret".to_string();
        let expiration = OffsetDateTime::now_utc() - Duration::hours(1); // expired
        let claims = Claims {
            sub: "user-123".into(),
            role: "Viewer".into(),
            exp: expiration.unix_timestamp(),
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap();
        
        let decode_result = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default());
        assert!(decode_result.is_err()); // Expired
    }
}
