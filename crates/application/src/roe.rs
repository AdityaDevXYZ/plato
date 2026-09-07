use crate::error::ApplicationError;
use release::{
    model::{Release, ReleaseVersion},
    repository::ReleaseRepository,
};
use types::{MissionId, ReleaseId, UserId};
use std::sync::Arc;
use eventbus::{EventPublisher, EventMessage};
use uuid::Uuid;

pub struct ReleaseOrchestrationService {
    release_repo: Arc<dyn ReleaseRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl ReleaseOrchestrationService {
    pub fn new(release_repo: Arc<dyn ReleaseRepository>, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { release_repo, event_publisher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn create_release(&self, mission_id: MissionId, version_str: String, actor: UserId, correlation_id: Uuid) -> Result<Release, ApplicationError> {
        let version = ReleaseVersion::new(version_str).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        let mut release = Release::new(version, mission_id, actor, correlation_id);
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    #[tracing::instrument(skip(self))]
    pub async fn validate_release(&self, id: ReleaseId, actor: UserId, correlation_id: Uuid, reason: String) -> Result<Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.validate(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    #[tracing::instrument(skip(self))]
    pub async fn submit_release(&self, id: ReleaseId, actor: UserId, correlation_id: Uuid, reason: String) -> Result<Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.submit(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    #[tracing::instrument(skip(self))]
    pub async fn approve_release(&self, id: ReleaseId, actor: UserId, correlation_id: Uuid, reason: String) -> Result<Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.approve(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    #[tracing::instrument(skip(self))]
    pub async fn mark_assured(&self, id: ReleaseId, actor: UserId, correlation_id: Uuid, reason: String) -> Result<Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.mark_assured(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    #[tracing::instrument(skip(self))]
    pub async fn mark_ready(&self, id: ReleaseId, actor: UserId, correlation_id: Uuid, reason: String) -> Result<Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.mark_ready(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    #[tracing::instrument(skip(self))]
    pub async fn begin_deployment(&self, id: ReleaseId, actor: UserId, correlation_id: Uuid, reason: String) -> Result<Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.begin_deployment(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    #[tracing::instrument(skip(self))]
    pub async fn complete_deployment(&self, id: ReleaseId, actor: UserId, correlation_id: Uuid, reason: String) -> Result<Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.complete_deployment(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }
    
    #[tracing::instrument(skip(self))]
    pub async fn get_release(&self, id: ReleaseId) -> Result<Release, ApplicationError> {
        self.fetch_release(id).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn fail_release(&self, id: ReleaseId, actor: types::UserId, correlation_id: Uuid, reason: String) -> Result<release::model::Release, ApplicationError> {
        let mut release = self.fetch_release(id).await?;
        release.fail(actor, correlation_id, reason).map_err(|e| ApplicationError::Domain(e.to_string()))?;
        self.save_and_publish(&mut release, correlation_id).await?;
        Ok(release)
    }

    async fn fetch_release(&self, id: ReleaseId) -> Result<Release, ApplicationError> {
        self.release_repo.find_by_id(id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Release not found".into()))
    }

    async fn save_and_publish(&self, release: &mut Release, correlation_id: Uuid) -> Result<(), ApplicationError> {
        let domain_events: Vec<EventMessage> = release.take_events().into_iter().map(|e| {
            EventMessage::new(e, correlation_id, "plato.roe")
        }).collect();
        self.release_repo.save(release, domain_events).await.map_err(|e| {
            match e {
                release::repository::RepositoryError::Concurrency => {
                        metrics::counter!("concurrency_conflicts_total").increment(1);
                        ApplicationError::Domain("Optimistic concurrency conflict".into())
                    },
                _ => ApplicationError::Repository(e.to_string())
            }
        })?;
        Ok(())
    }
}
