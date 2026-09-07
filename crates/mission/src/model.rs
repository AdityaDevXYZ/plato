use types::{MissionId, ProjectId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MissionError {
    #[error("Mission name cannot be empty")]
    EmptyName,
    #[error("Invalid state transition")]
    InvalidTransition,
}

/// A value object representing a validated mission name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionName(String);

impl MissionName {
    pub fn new(name: impl Into<String>) -> Result<Self, MissionError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(MissionError::EmptyName);
        }
        Ok(Self(name))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissionStatus {
    Planning,
    Active,
    Completed,
}

/// Represents a Mission entity.
#[derive(Debug, Clone)]
pub struct Mission {
    id: MissionId,
    project_id: ProjectId,
    name: MissionName,
    status: MissionStatus,
}

impl Mission {
    /// Constructs a new Mission in planning phase.
    
    pub fn reconstruct(id: types::MissionId, name: MissionName, project_id: types::ProjectId, status: MissionStatus) -> Self {
        Self { id, project_id, name, status }
    }
    
    pub fn new(name: MissionName, project_id: types::ProjectId) -> Self {
        Self {
            id: MissionId::new(),
            project_id,
            name,
            status: MissionStatus::Planning,
        }
    }

    pub fn id(&self) -> MissionId { self.id }
    pub fn project_id(&self) -> ProjectId { self.project_id }
    pub fn name(&self) -> &MissionName { &self.name }
    pub fn status(&self) -> &MissionStatus { &self.status }

    pub fn start(&mut self) -> Result<(), MissionError> {
        if self.status != MissionStatus::Planning {
            return Err(MissionError::InvalidTransition);
        }
        self.status = MissionStatus::Active;
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), MissionError> {
        if self.status != MissionStatus::Active {
            return Err(MissionError::InvalidTransition);
        }
        self.status = MissionStatus::Completed;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name() {
        assert!(MissionName::new("Alpha").is_ok());
    }

    #[test]
    fn test_mission_flow() {
        let name = MissionName::new("Alpha").unwrap();
        let mut mission = Mission::new(name, ProjectId::new());
        assert_eq!(mission.status(), &MissionStatus::Planning);
        
        assert!(mission.start().is_ok());
        assert_eq!(mission.status(), &MissionStatus::Active);

        assert!(mission.complete().is_ok());
        assert_eq!(mission.status(), &MissionStatus::Completed);
    }
}
