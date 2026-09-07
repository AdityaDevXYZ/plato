use types::{OrganizationId, UserId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrgError {
    #[error("Organization name cannot be empty")]
    EmptyName,
    #[error("Organization is already suspended")]
    AlreadySuspended,
}

/// A value object representing a validated organization name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationName(String);

impl OrganizationName {
    pub fn new(name: impl Into<String>) -> Result<Self, OrgError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(OrgError::EmptyName);
        }
        Ok(Self(name))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationStatus {
    Active,
    Suspended,
}

/// Represents an Organization entity.
#[derive(Debug, Clone)]
pub struct Organization {
    id: OrganizationId,
    name: OrganizationName,
    owner_id: UserId,
    status: OrganizationStatus,
}

impl Organization {
    /// Constructs a new active Organization.
    
    pub fn reconstruct(id: types::OrganizationId, name: OrganizationName, owner_id: types::UserId, status: OrganizationStatus) -> Self {
        Self { id, name, owner_id, status }
    }
    
    pub fn new(name: OrganizationName, owner_id: types::UserId) -> Self {
        Self {
            id: OrganizationId::new(),
            name,
            owner_id,
            status: OrganizationStatus::Active,
        }
    }

    pub fn id(&self) -> OrganizationId { self.id }
    pub fn name(&self) -> &OrganizationName { &self.name }
    pub fn owner_id(&self) -> UserId { self.owner_id }
    pub fn status(&self) -> &OrganizationStatus { &self.status }

    /// Suspends the organization.
    pub fn suspend(&mut self) -> Result<(), OrgError> {
        if self.status == OrganizationStatus::Suspended {
            return Err(OrgError::AlreadySuspended);
        }
        self.status = OrganizationStatus::Suspended;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name() {
        assert!(OrganizationName::new("Acme Corp").is_ok());
    }

    #[test]
    fn invalid_name() {
        assert!(OrganizationName::new("   ").is_err());
    }

    #[test]
    fn test_organization_suspend() {
        let name = OrganizationName::new("Acme").unwrap();
        let mut org = Organization::new(name, UserId::new());
        assert!(org.suspend().is_ok());
        assert_eq!(org.status(), &OrganizationStatus::Suspended);
        assert!(org.suspend().is_err());
    }
}
