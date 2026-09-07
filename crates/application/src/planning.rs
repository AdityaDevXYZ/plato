use crate::error::ApplicationError;
use crate::twin::TwinQueryService;
use crate::orbit::OrbitalQueryService;
use planning::{
    model::*,
    strategy::PlanningStrategy,
    repository::PlanningRepository,
    events::*,
};
use types::{MissionId, ReleaseId};
use std::sync::Arc;
use uuid::Uuid;
use eventbus::{EventPublisher, EventMessage};

pub struct MissionPlanningService {
    repo: Arc<dyn PlanningRepository>,
    twin_query: Arc<TwinQueryService>,
    _orbit_query: Arc<OrbitalQueryService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl MissionPlanningService {
    pub fn new(repo: Arc<dyn PlanningRepository>, twin_query: Arc<TwinQueryService>, _orbit_query: Arc<OrbitalQueryService>, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { repo, twin_query, _orbit_query, event_publisher }
    }

    #[tracing::instrument(skip(self, strategy))]
    pub async fn generate_plan(&self, mission_id: MissionId, release_id: ReleaseId, strategy: &dyn PlanningStrategy, correlation_id: Uuid) -> Result<DeploymentPlan, ApplicationError> {
        let sats = self.twin_query.get_mission_assets(mission_id).await?;
        let active_sats: Vec<Uuid> = sats.into_iter().map(|s| s.id).collect();

        let ctx = PlanningContext { available_satellites: active_sats };

        let plan = match strategy.generate_plan(mission_id, release_id, correlation_id, &ctx).await {
            Ok(p) => p,
            Err(e) => {
                let msg = EventMessage::new(Box::new(PlanningFailed { mission_id, release_id, reason: e.clone() }), correlation_id, "plato.planning");
                let _ = self.event_publisher.publish(vec![msg]);
                return Err(ApplicationError::Domain(format!("Planning strategy failed: {}", e)));
            }
        };

        self.repo.save_plan(&plan).await.map_err(|e| ApplicationError::Repository(e.to_string()))?;

        let msg = EventMessage::new(Box::new(DeploymentPlanCreated { plan_id: plan.plan_id, mission_id, release_id }), correlation_id, "plato.planning");
        let _ = self.event_publisher.publish(vec![msg]);

        Ok(plan)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_plan(&self, plan_id: Uuid) -> Result<DeploymentPlan, ApplicationError> {
        self.repo.get_plan(plan_id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Plan not found".into()))
    }
}
