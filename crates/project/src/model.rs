use types::{ProjectId, OrganizationId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Project name cannot be empty")]
    EmptyName,
    #[error("Project is already archived")]
    AlreadyArchived,
}

/// A value object representing a validated project name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(name: impl Into<String>) -> Result<Self, ProjectError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ProjectError::EmptyName);
        }
        Ok(Self(name))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Active,
    Archived,
}

/// Represents a Project entity.
#[derive(Debug, Clone)]
pub struct Project {
    id: ProjectId,
    org_id: OrganizationId,
    name: ProjectName,
    status: ProjectStatus,
}

impl Project {
    /// Constructs a new active Project.
    
    pub fn reconstruct(id: types::ProjectId, name: ProjectName, org_id: types::OrganizationId, status: ProjectStatus) -> Self {
        Self { id, org_id, name, status }
    }
    
    pub fn new(name: ProjectName, org_id: types::OrganizationId) -> Self {
        Self {
            id: ProjectId::new(),
            org_id,
            name,
            status: ProjectStatus::Active,
        }
    }

    pub fn id(&self) -> ProjectId { self.id }
    pub fn org_id(&self) -> OrganizationId { self.org_id }
    pub fn name(&self) -> &ProjectName { &self.name }
    pub fn status(&self) -> &ProjectStatus { &self.status }

    /// Archives the project.
    pub fn archive(&mut self) -> Result<(), ProjectError> {
        if self.status == ProjectStatus::Archived {
            return Err(ProjectError::AlreadyArchived);
        }
        self.status = ProjectStatus::Archived;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name() {
        assert!(ProjectName::new("Project Apollo").is_ok());
    }

    #[test]
    fn invalid_name() {
        assert!(ProjectName::new("   ").is_err());
    }

    #[test]
    fn test_project_archive() {
        let name = ProjectName::new("Apollo").unwrap();
        let mut proj = Project::new(name, OrganizationId::new());
        assert!(proj.archive().is_ok());
        assert_eq!(proj.status(), &ProjectStatus::Archived);
        assert!(proj.archive().is_err());
    }
}
