use async_trait::async_trait;
use sqlx::PgPool;
use mission::{model::{Mission, MissionStatus, MissionName}, repository::{MissionRepository, RepositoryError}};
use types::MissionId;
use uuid::Uuid;

pub struct SqlxMissionRepository {
    pool: PgPool,
}

impl SqlxMissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct Row {
    id: Uuid,
    project_id: Uuid,
    name: String,
    status: String,
}

#[async_trait]
impl MissionRepository for SqlxMissionRepository {
    async fn save(&self, mission: &Mission) -> Result<(), RepositoryError> {
        let id: Uuid = mission.id().into();
        let project_id: Uuid = mission.project_id().into();
        let name: &str = mission.name().as_str();
        let status = format!("{:?}", mission.status());

        sqlx::query("INSERT INTO missions (id, project_id, name, status) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET project_id = $2, name = $3, status = $4")
            .bind(id)
            .bind(project_id)
            .bind(name)
            .bind(status)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: MissionId) -> Result<Option<Mission>, RepositoryError> {
        let uuid: Uuid = id.into();
        let row = sqlx::query_as::<_, Row>("SELECT id, project_id, name, status FROM missions WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if let Some(r) = row {
            let status = match r.status.as_str() {
                "Active" => MissionStatus::Active,
                "Completed" => MissionStatus::Completed,
                _ => MissionStatus::Planning,
            };
            let name = MissionName::new(r.name)
                .map_err(|e| RepositoryError::Database(format!("{:?}", e)))?;
            Ok(Some(Mission::reconstruct(r.id.into(), name, r.project_id.into(), status)))
        } else {
            Ok(None)
        }
    }
}
