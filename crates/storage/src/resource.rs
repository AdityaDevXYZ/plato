use async_trait::async_trait;
use sqlx::PgPool;
use resource::{model::{SatelliteResource, GroundStationResource}, repository::{ResourceRepository, RepositoryError}};
use uuid::Uuid;

pub struct SqlxResourceRepository { pool: PgPool }
impl SqlxResourceRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, data: serde_json::Value }

#[async_trait]
impl ResourceRepository for SqlxResourceRepository {
    async fn save_satellite_resource(&self, res: &SatelliteResource) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(res).unwrap();
        sqlx::query("INSERT INTO satellite_resources (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $2")
            .bind(res.satellite_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_satellite_resource(&self, id: Uuid) -> Result<Option<SatelliteResource>, RepositoryError> {
        let row = sqlx::query_as::<_, Row>("SELECT id, data FROM satellite_resources WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row { Ok(Some(serde_json::from_value(r.data).unwrap())) } else { Ok(None) }
    }
    async fn get_all_satellite_resources(&self) -> Result<Vec<SatelliteResource>, RepositoryError> {
        let rows = sqlx::query_as::<_, Row>("SELECT id, data FROM satellite_resources")
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
    async fn save_ground_station_resource(&self, res: &GroundStationResource) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(res).unwrap();
        sqlx::query("INSERT INTO ground_station_resources (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $2")
            .bind(res.ground_station_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_all_ground_station_resources(&self) -> Result<Vec<GroundStationResource>, RepositoryError> {
        let rows = sqlx::query_as::<_, Row>("SELECT id, data FROM ground_station_resources")
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
}
