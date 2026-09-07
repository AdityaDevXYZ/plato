use types::ReleaseId;
use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentState {
    Initializing, Executing, Waiting, Retrying, RollingBack, Succeeded, Failed, Cancelled
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackStatus { None, InProgress, Succeeded, Failed }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSession {
    pub session_id: Uuid,
    pub release_id: ReleaseId,
    pub correlation_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub started_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601::option")]
    pub completed_at: Option<OffsetDateTime>,
    pub status: DeploymentState,
    pub execution_log: Vec<LogEntry>,
    pub failure_reason: Option<String>,
    pub rollback_status: RollbackStatus,
}

impl DeploymentSession {
    pub fn new(release_id: ReleaseId, correlation_id: Uuid) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            release_id,
            correlation_id,
            started_at: OffsetDateTime::now_utc(),
            completed_at: None,
            status: DeploymentState::Initializing,
            execution_log: Vec::new(),
            failure_reason: None,
            rollback_status: RollbackStatus::None,
        }
    }
    
    pub fn log(&mut self, level: &str, message: &str) {
        self.execution_log.push(LogEntry {
            timestamp: OffsetDateTime::now_utc(),
            level: level.into(),
            message: message.into(),
        });
    }
}
