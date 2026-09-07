use async_trait::async_trait;
use sqlx::PgPool;
use orbit::{model::{SatelliteOrbit, OrbitalPrediction, CommunicationWindow}, repository::{OrbitalRepository, RepositoryError}};
use uuid::Uuid;
use time::OffsetDateTime;

pub struct SqlxOrbitRepository { pool: PgPool }
impl SqlxOrbitRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, data: serde_json::Value }

#[derive(sqlx::FromRow)]
struct CommRow { data: serde_json::Value }

#[async_trait]
impl OrbitalRepository for SqlxOrbitRepository {
    async fn save_orbit(&self, orbit: &SatelliteOrbit) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(orbit).unwrap();
        sqlx::query("INSERT INTO satellite_orbits (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $2")
            .bind(orbit.satellite_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_orbit(&self, id: Uuid) -> Result<Option<SatelliteOrbit>, RepositoryError> {
        let row = sqlx::query_as::<_, Row>("SELECT id, data FROM satellite_orbits WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row { Ok(Some(serde_json::from_value(r.data).unwrap())) } else { Ok(None) }
    }
    async fn save_prediction(&self, pred: &OrbitalPrediction) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(pred).unwrap();
        sqlx::query("INSERT INTO orbital_predictions (id, target_time, data) VALUES ($1, $2, $3)")
            .bind(pred.satellite_id)
            .bind(pred.target_time)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_predictions(&self, id: Uuid) -> Result<Vec<OrbitalPrediction>, RepositoryError> {
        let rows = sqlx::query_as::<_, Row>("SELECT id, data FROM orbital_predictions WHERE id = $1")
            .bind(id)
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
    async fn save_window(&self, window: &CommunicationWindow) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(window).unwrap();
        sqlx::query("INSERT INTO comm_windows (sat_id, ground_id, start_time, end_time, data) VALUES ($1, $2, $3, $4, $5)")
            .bind(window.satellite_id)
            .bind(window.ground_station_id)
            .bind(window.start_time)
            .bind(window.end_time)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_windows(&self, after: OffsetDateTime) -> Result<Vec<CommunicationWindow>, RepositoryError> {
        let rows = sqlx::query_as::<_, CommRow>("SELECT data FROM comm_windows WHERE end_time > $1")
            .bind(after)
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
}
