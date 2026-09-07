use crate::model::*;
use types::{MissionId, ReleaseId};
use uuid::Uuid;
use async_trait::async_trait;
use time::OffsetDateTime;

#[async_trait]
pub trait PlanningStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    async fn generate_plan(&self, mission_id: MissionId, release_id: ReleaseId, correlation_id: Uuid, context: &PlanningContext) -> Result<DeploymentPlan, String>;
}

pub struct SequentialRollout;
#[async_trait]
impl PlanningStrategy for SequentialRollout {
    fn name(&self) -> &'static str { "SequentialRollout" }
    async fn generate_plan(&self, mission_id: MissionId, release_id: ReleaseId, correlation_id: Uuid, context: &PlanningContext) -> Result<DeploymentPlan, String> {
        let mut phases = Vec::new();
        let mut prev_phase_id = None;
        
        for (i, sat_id) in context.available_satellites.iter().enumerate() {
            let phase_id = Uuid::new_v4();
            let mut depends_on = Vec::new();
            if let Some(id) = prev_phase_id {
                depends_on.push(id);
            }
            
            phases.push(ExecutionPhase {
                phase_id,
                name: format!("Phase {}", i + 1),
                order: (i + 1) as u32,
                target_satellites: vec![*sat_id],
                suggested_window: None,
                depends_on,
            });
            prev_phase_id = Some(phase_id);
        }

        Ok(DeploymentPlan {
            plan_id: Uuid::new_v4(),
            mission_id, release_id, correlation_id,
            created_at: OffsetDateTime::now_utc(),
            planning_horizon_sec: 86400,
            target_assets: context.available_satellites.clone(),
            phases,
            priority: 1,
            risk_summary: RiskSummary { risk_level: "LOW".into(), identified_risks: vec![] }
        })
    }
}

pub struct CanaryRollout;
#[async_trait]
impl PlanningStrategy for CanaryRollout {
    fn name(&self) -> &'static str { "CanaryRollout" }
    async fn generate_plan(&self, mission_id: MissionId, release_id: ReleaseId, correlation_id: Uuid, context: &PlanningContext) -> Result<DeploymentPlan, String> {
        let mut phases = Vec::new();
        if context.available_satellites.is_empty() {
            return Err("No satellites available for canary".into());
        }
        
        let canary_phase_id = Uuid::new_v4();
        phases.push(ExecutionPhase {
            phase_id: canary_phase_id,
            name: "Canary Deployment".into(),
            order: 1,
            target_satellites: vec![context.available_satellites[0]],
            suggested_window: None,
            depends_on: vec![],
        });
        
        if context.available_satellites.len() > 1 {
            phases.push(ExecutionPhase {
                phase_id: Uuid::new_v4(),
                name: "Main Deployment".into(),
                order: 2,
                target_satellites: context.available_satellites[1..].to_vec(),
                suggested_window: None,
                depends_on: vec![canary_phase_id],
            });
        }

        Ok(DeploymentPlan {
            plan_id: Uuid::new_v4(),
            mission_id, release_id, correlation_id,
            created_at: OffsetDateTime::now_utc(),
            planning_horizon_sec: 86400 * 2,
            target_assets: context.available_satellites.clone(),
            phases,
            priority: 2,
            risk_summary: RiskSummary { risk_level: "MEDIUM".into(), identified_risks: vec!["Canary failure blocks main".into()] }
        })
    }
}

pub struct BatchRollout;
#[async_trait]
impl PlanningStrategy for BatchRollout {
    fn name(&self) -> &'static str { "BatchRollout" }
    async fn generate_plan(&self, mission_id: MissionId, release_id: ReleaseId, correlation_id: Uuid, context: &PlanningContext) -> Result<DeploymentPlan, String> {
        Ok(DeploymentPlan {
            plan_id: Uuid::new_v4(),
            mission_id, release_id, correlation_id,
            created_at: OffsetDateTime::now_utc(),
            planning_horizon_sec: 43200,
            target_assets: context.available_satellites.clone(),
            phases: vec![ExecutionPhase {
                phase_id: Uuid::new_v4(),
                name: "Global Batch".into(),
                order: 1,
                target_satellites: context.available_satellites.clone(),
                suggested_window: None,
                depends_on: vec![],
            }],
            priority: 3,
            risk_summary: RiskSummary { risk_level: "HIGH".into(), identified_risks: vec!["Global failure risk".into()] }
        })
    }
}
