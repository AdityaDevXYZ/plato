use types::{ReleaseId, MissionId, UserId};
use thiserror::Error;
use eventbus::Event;
use crate::events::*;
use time::OffsetDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ReleaseError {
    #[error("Concurrency error: release modified by another transaction")]
    ConcurrencyError,

    #[error("Release version cannot be empty or invalid format")]
    InvalidVersion,
    #[error("Invalid state transition from {0:?} to {1:?}")]
    InvalidTransition(ReleaseStatus, ReleaseStatus),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum ReleaseStatus { 
    Draft, Validated, Submitted, Approved, Assured, Ready, Deploying, Completed, Failed, Cancelled 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRecord {
    pub from: ReleaseStatus,
    pub to: ReleaseStatus,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub actor: Uuid,
    pub correlation_id: Uuid,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseVersion(String);
impl ReleaseVersion {
    pub fn new(version: impl Into<String>) -> Result<Self, ReleaseError> {
        let version = version.into();
        if version.trim().is_empty() || !version.contains('.') { return Err(ReleaseError::InvalidVersion); }
        Ok(Self(version))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

pub struct Release {
    pub version_num: u64,
    id: ReleaseId,
    mission_id: MissionId,
    version: ReleaseVersion,
    status: ReleaseStatus,
    history: Vec<TransitionRecord>,
    pending_events: Vec<Box<dyn Event>>,
}

impl Release {
    pub fn reconstruct(id: ReleaseId, mission_id: MissionId, version: ReleaseVersion, status: ReleaseStatus, history: Vec<TransitionRecord>, version_num: u64) -> Self {
        Self { id, mission_id, version, status, history, version_num, pending_events: Vec::new() }
    }
    
    pub fn new(version: ReleaseVersion, mission_id: MissionId, actor: UserId, correlation_id: Uuid) -> Self {
        let id = ReleaseId::new();
        Self {
            id,
            mission_id,
            version,
            status: ReleaseStatus::Draft,
            history: Vec::new(),
            version_num: 1,
            pending_events: vec![Box::new(ReleaseCreated { release_id: id })],
        }
    }

    pub fn id(&self) -> ReleaseId { self.id }
    pub fn mission_id(&self) -> MissionId { self.mission_id }
    pub fn version(&self) -> &ReleaseVersion { &self.version }
    pub fn status(&self) -> ReleaseStatus { self.status }
    pub fn history(&self) -> &[TransitionRecord] { &self.history }
    
    pub fn take_events(&mut self) -> Vec<Box<dyn Event>> {
        std::mem::take(&mut self.pending_events)
    }

    fn transition(&mut self, to: ReleaseStatus, actor: UserId, correlation_id: Uuid, reason: impl Into<String>, event: Box<dyn Event>) -> Result<(), ReleaseError> {
        let from = self.status;
        self.status = to;
        self.history.push(TransitionRecord {
            from,
            to,
            timestamp: OffsetDateTime::now_utc(),
            actor: actor.into(),
            correlation_id,
            reason: reason.into(),
        });
        self.pending_events.push(event);
        Ok(())
    }

    pub fn validate(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        if self.status != ReleaseStatus::Draft { return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Validated)); }
        self.transition(ReleaseStatus::Validated, actor, correlation_id, reason, Box::new(ReleaseValidated { release_id: self.id }))
    }

    pub fn submit(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        if self.status != ReleaseStatus::Validated { return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Submitted)); }
        self.transition(ReleaseStatus::Submitted, actor, correlation_id, reason, Box::new(ReleaseSubmitted { release_id: self.id }))
    }

    pub fn approve(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        if self.status != ReleaseStatus::Submitted { return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Approved)); }
        self.transition(ReleaseStatus::Approved, actor, correlation_id, reason, Box::new(ReleaseApproved { release_id: self.id }))
    }

    pub fn mark_assured(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        if self.status != ReleaseStatus::Approved { return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Assured)); }
        self.transition(ReleaseStatus::Assured, actor, correlation_id, reason, Box::new(ReleaseAssured { release_id: self.id }))
    }

    pub fn mark_ready(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        if self.status != ReleaseStatus::Assured { return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Ready)); }
        self.transition(ReleaseStatus::Ready, actor, correlation_id, reason, Box::new(ReleaseReady { release_id: self.id }))
    }

    pub fn begin_deployment(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        if self.status != ReleaseStatus::Ready { return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Deploying)); }
        self.transition(ReleaseStatus::Deploying, actor, correlation_id, reason, Box::new(ReleaseDeploymentStarted { release_id: self.id }))
    }

    pub fn complete_deployment(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        if self.status != ReleaseStatus::Deploying { return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Completed)); }
        self.transition(ReleaseStatus::Completed, actor, correlation_id, reason, Box::new(ReleaseCompleted { release_id: self.id }))
    }

    pub fn fail(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        self.transition(ReleaseStatus::Failed, actor, correlation_id, reason, Box::new(ReleaseFailed { release_id: self.id }))
    }

    pub fn cancel(&mut self, actor: UserId, correlation_id: Uuid, reason: String) -> Result<(), ReleaseError> {
        match self.status {
            ReleaseStatus::Completed | ReleaseStatus::Failed | ReleaseStatus::Cancelled => {
                return Err(ReleaseError::InvalidTransition(self.status, ReleaseStatus::Cancelled));
            }
            _ => {}
        }
        self.transition(ReleaseStatus::Cancelled, actor, correlation_id, reason, Box::new(ReleaseCancelled { release_id: self.id }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let mut r = Release::new(ReleaseVersion::new("1.0.0").unwrap(), MissionId::new(), UserId::new(), Uuid::new_v4());
        assert_eq!(r.status(), ReleaseStatus::Draft);
        assert!(r.validate(UserId::new(), Uuid::new_v4(), "ok".into()).is_ok());
        assert_eq!(r.status(), ReleaseStatus::Validated);
        assert!(r.submit(UserId::new(), Uuid::new_v4(), "ok".into()).is_ok());
        assert!(r.approve(UserId::new(), Uuid::new_v4(), "ok".into()).is_ok());
        assert!(r.mark_assured(UserId::new(), Uuid::new_v4(), "ok".into()).is_ok());
        assert!(r.mark_ready(UserId::new(), Uuid::new_v4(), "ok".into()).is_ok());
        assert!(r.begin_deployment(UserId::new(), Uuid::new_v4(), "ok".into()).is_ok());
        assert!(r.complete_deployment(UserId::new(), Uuid::new_v4(), "ok".into()).is_ok());
        
        // Assert Audit history length
        assert_eq!(r.history().len(), 7); // validate -> submit -> approve -> assured -> ready -> deploy -> complete
    }

    #[test]
    fn test_invalid_transitions() {
        let mut r = Release::new(ReleaseVersion::new("1.0.0").unwrap(), MissionId::new(), UserId::new(), Uuid::new_v4());
        assert!(r.submit(UserId::new(), Uuid::new_v4(), "ok".into()).is_err());
        assert!(r.approve(UserId::new(), Uuid::new_v4(), "ok".into()).is_err());
    }
}
