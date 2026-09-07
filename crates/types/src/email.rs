use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Error)]
#[error("Invalid email format")]
pub struct EmailError;

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into();
        if !email.contains('@') || !email.contains('.') {
            return Err(EmailError);
        }
        Ok(Self(email))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email() {
        assert!(Email::new("test@example.com").is_ok());
    }

    #[test]
    fn invalid_email() {
        assert!(Email::new("invalid").is_err());
    }
}
