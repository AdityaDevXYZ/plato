use crate::error::ApplicationError;
use crate::planning::MissionPlanningService;
use coordination::{
    model::*,
    repository::CoordinationRepository,
    events::*,
};
use std::sync::Arc;
use uuid::Uuid;
use eventbus::{EventPublisher, EventMessage};
use time::OffsetDateTime;
use std::collections::HashMap;

pub struct CoordinationService {
    repo: Arc<dyn CoordinationRepository>,
    planning_service: Arc<MissionPlanningService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl CoordinationService {
    pub fn new(repo: Arc<dyn CoordinationRepository>, planning_service: Arc<MissionPlanningService>, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { repo, planning_service, event_publisher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn start_coordination(&self, plan_id: Uuid, correlation_id: Uuid) -> Result<CoordinationSession, ApplicationError> {
        let plan = self.planning_service.get_plan(plan_id).await?;
        
        let mut waves = Vec::new();
        let mut phase_to_wave = HashMap::new();
        
        // Translate Plan Phases into Execution Waves
        for phase in &plan.phases {
            let wave_id = Uuid::new_v4();
            phase_to_wave.insert(phase.phase_id, wave_id);
            
            let dependencies: Vec<WaveDependency> = phase.depends_on.iter().filter_map(|p_id| {
                phase_to_wave.get(p_id).map(|&w_id| WaveDependency { required_wave_id: w_id })
            }).collect();
            
            waves.push(ExecutionWave {
                wave_id,
                name: phase.name.clone(),
                target_satellites: phase.target_satellites.clone(),
                dependencies,
                status: WaveStatus::Pending,
            });
        }

        let session = CoordinationSession {
            session_id: Uuid::new_v4(),
            plan_id,
            release_id: plan.release_id,
            correlation_id,
            created_at: OffsetDateTime::now_utc(),
            waves,
            status: CoordinationStatus::Pending,
        };

        self.repo.save_session(&session).await.map_err(|e| ApplicationError::Repository(e.to_string()))?;

        let msg = EventMessage::new(Box::new(CoordinationStarted { session_id: session.session_id, plan_id, release_id: plan.release_id }), correlation_id, "plato.coordination");
        let _ = self.event_publisher.publish(vec![msg]);

        // In a real system, a worker would poll pending sessions and mark them InProgress, checking dependencies and emitting WaveStarted.
        
        Ok(session)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_session(&self, id: Uuid) -> Result<CoordinationSession, ApplicationError> {
        self.repo.get_session(id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Coordination session not found".into()))
    }
}
