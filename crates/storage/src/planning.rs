use async_trait::async_trait;
use sqlx::PgPool;
use planning::{model::DeploymentPlan, repository::{PlanningRepository, RepositoryError}};
use types::MissionId;
use uuid::Uuid;

pub struct SqlxPlanningRepository { pool: PgPool }
impl SqlxPlanningRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, mission_id: Uuid, data: serde_json::Value }

#[async_trait]
impl PlanningRepository for SqlxPlanningRepository {
    async fn save_plan(&self, plan: &DeploymentPlan) -> Result<(), RepositoryError> {
        let mid: Uuid = plan.mission_id.into();
        let data = serde_json::to_value(plan).unwrap();
        sqlx::query("INSERT INTO deployment_plans (id, mission_id, data) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET data = $3")
            .bind(plan.plan_id)
            .bind(mid)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_plan(&self, plan_id: Uuid) -> Result<Option<DeploymentPlan>, RepositoryError> {
        let row = sqlx::query_as::<_, Row>("SELECT id, mission_id, data FROM deployment_plans WHERE id = $1")
            .bind(plan_id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row { Ok(Some(serde_json::from_value(r.data).unwrap())) } else { Ok(None) }
    }
    async fn get_plans_by_mission(&self, mission_id: MissionId) -> Result<Vec<DeploymentPlan>, RepositoryError> {
        let mid: Uuid = mission_id.into();
        let rows = sqlx::query_as::<_, Row>("SELECT id, mission_id, data FROM deployment_plans WHERE mission_id = $1")
            .bind(mid)
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
}
