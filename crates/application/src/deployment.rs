use crate::error::ApplicationError;
use deployment::{
    model::{DeploymentSession, DeploymentState, RollbackStatus},
    pipeline::{DeploymentStep, RollbackStrategy, DeploymentContext},
    repository::DeploymentSessionRepository,
    events::*,
};
use types::ReleaseId;
use std::sync::Arc;
use eventbus::{EventPublisher, EventMessage};
use uuid::Uuid;
use time::OffsetDateTime;

pub struct DeploymentExecutionService {
    steps: Vec<Arc<dyn DeploymentStep>>,
    rollback_strategy: Arc<dyn RollbackStrategy>,
    session_repo: Arc<dyn DeploymentSessionRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl DeploymentExecutionService {
    pub fn new(steps: Vec<Arc<dyn DeploymentStep>>, rollback_strategy: Arc<dyn RollbackStrategy>, session_repo: Arc<dyn DeploymentSessionRepository>, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { steps, rollback_strategy, session_repo, event_publisher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn execute_deployment(&self, release_id: ReleaseId, correlation_id: Uuid) -> Result<DeploymentSession, ApplicationError> {
        let mut session = DeploymentSession::new(release_id, correlation_id);
        let ctx = DeploymentContext { release_id, session_id: session.session_id };

        session.status = DeploymentState::Executing;
        session.log("INFO", "Deployment started");
        self.session_repo.save(&session).await.map_err(|e| ApplicationError::Repository(e.to_string()))?;

        let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentStarted { session_id: session.session_id, release_id }), correlation_id, "plato.dee")]);

        let mut failed_step: Option<(String, String)> = None;

        for step in &self.steps {
            let name = step.name();
            session.log("INFO", &format!("Executing step: {}", name));
            self.session_repo.save(&session).await.ok();
            
            let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentProgressUpdated { session_id: session.session_id, step: name.into() }), correlation_id, "plato.dee")]);

            let policy = step.retry_policy();
            let mut attempt = 0;
            let mut success = false;
            
            while attempt <= policy.max_retries {
                if attempt > 0 {
                    session.status = DeploymentState::Retrying;
                    session.log("WARN", &format!("Retrying step {} (attempt {})", name, attempt));
                    self.session_repo.save(&session).await.ok();
                    let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentRetrying { session_id: session.session_id, step: name.into(), attempt }), correlation_id, "plato.dee")]);
                }
                
                match step.execute(&ctx).await {
                    Ok(_) => {
                        success = true;
                        break;
                    }
                    Err(e) => {
                        session.log("ERROR", &format!("Step {} failed: {}", name, e));
                        if attempt == policy.max_retries {
                            failed_step = Some((name.into(), e));
                        }
                    }
                }
                attempt += 1;
            }
            
            if !success { break; }
        }

        if let Some((failed_name, reason)) = failed_step {
            session.status = DeploymentState::RollingBack;
            session.rollback_status = RollbackStatus::InProgress;
            let fail_reason = format!("Failed at step {}: {}", failed_name, reason);
            session.failure_reason = Some(fail_reason.clone());
            session.log("ERROR", "Initiating rollback");
            self.session_repo.save(&session).await.ok();

            let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentRollbackStarted { session_id: session.session_id, reason: fail_reason.clone() }), correlation_id, "plato.dee")]);

            match self.rollback_strategy.invoke_rollback(&ctx, &reason).await {
                Ok(_) => {
                    session.rollback_status = RollbackStatus::Succeeded;
                    session.log("INFO", "Rollback succeeded");
                    let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentRollbackCompleted { session_id: session.session_id, success: true }), correlation_id, "plato.dee")]);
                }
                Err(e) => {
                    session.rollback_status = RollbackStatus::Failed;
                    session.log("ERROR", &format!("Rollback failed: {}", e));
                    let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentRollbackCompleted { session_id: session.session_id, success: false }), correlation_id, "plato.dee")]);
                }
            }

            session.status = DeploymentState::Failed;
            session.completed_at = Some(OffsetDateTime::now_utc());
            self.session_repo.save(&session).await.map_err(|e| ApplicationError::Repository(e.to_string()))?;
            
            let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentFailed { session_id: session.session_id, release_id, reason: fail_reason }), correlation_id, "plato.dee")]);

        } else {
            session.status = DeploymentState::Succeeded;
            session.completed_at = Some(OffsetDateTime::now_utc());
            session.log("INFO", "Deployment succeeded");
            self.session_repo.save(&session).await.map_err(|e| ApplicationError::Repository(e.to_string()))?;
            
            let _ = self.event_publisher.publish(vec![EventMessage::new(Box::new(DeploymentSucceeded { session_id: session.session_id, release_id }), correlation_id, "plato.dee")]);
        }

        Ok(session)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_session(&self, session_id: Uuid) -> Result<DeploymentSession, ApplicationError> {
        self.session_repo.find_by_id(session_id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Session not found".into()))
    }
}
