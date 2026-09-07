use async_trait::async_trait;
use sqlx::PgPool;
use twin::{model::{Satellite, OperationalStatus, CommunicationStatus}, repository::{TwinRepository, RepositoryError}};
use types::MissionId;
use uuid::Uuid;

pub struct SqlxDigitalTwinRepository { pool: PgPool }
impl SqlxDigitalTwinRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct SatRow { id: Uuid, mission_id: Uuid, data: serde_json::Value }

#[async_trait]
impl TwinRepository for SqlxDigitalTwinRepository {
    async fn save_satellite(&self, sat: &Satellite) -> Result<(), RepositoryError> {
        let mission_id: Uuid = sat.mission_id.into();
        let data = serde_json::to_value(sat).unwrap();
        sqlx::query("INSERT INTO twin_satellites (id, mission_id, data) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET mission_id = $2, data = $3")
            .bind(sat.id)
            .bind(mission_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_satellite(&self, id: Uuid) -> Result<Option<Satellite>, RepositoryError> {
        let row = sqlx::query_as::<_, SatRow>("SELECT id, mission_id, data FROM twin_satellites WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row { Ok(Some(serde_json::from_value(r.data).unwrap())) } else { Ok(None) }
    }
    async fn get_mission_assets(&self, mission_id: MissionId) -> Result<Vec<Satellite>, RepositoryError> {
        let mid: Uuid = mission_id.into();
        let rows = sqlx::query_as::<_, SatRow>("SELECT id, mission_id, data FROM twin_satellites WHERE mission_id = $1")
            .bind(mid)
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
    async fn get_all_satellites(&self) -> Result<Vec<Satellite>, RepositoryError> {
        let rows = sqlx::query_as::<_, SatRow>("SELECT id, mission_id, data FROM twin_satellites")
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
    async fn get_unhealthy_satellites(&self, threshold: u8) -> Result<Vec<Satellite>, RepositoryError> {
        let all = self.get_all_satellites().await?;
        Ok(all.into_iter().filter(|s| s.health_score < threshold).collect())
    }
    async fn get_online_satellites(&self) -> Result<Vec<Satellite>, RepositoryError> {
        let all = self.get_all_satellites().await?;
        Ok(all.into_iter().filter(|s| s.operational_status == OperationalStatus::Active && s.comm_status == CommunicationStatus::Connected).collect())
    }
    async fn get_deployable_satellites(&self) -> Result<Vec<Satellite>, RepositoryError> {
        let all = self.get_all_satellites().await?;
        Ok(all.into_iter().filter(|s| s.operational_status == OperationalStatus::Active && s.health_score >= 80).collect())
    }
}
