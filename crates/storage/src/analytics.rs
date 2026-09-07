use async_trait::async_trait;
use sqlx::PgPool;
use analytics::{model::{DeploymentAnalyticsReport, StrategyPerformance, SatelliteReliability}, repository::{AnalyticsRepository, RepositoryError}};
use uuid::Uuid;

pub struct SqlxAnalyticsRepository { pool: PgPool }
impl SqlxAnalyticsRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, data: serde_json::Value }

#[async_trait]
impl AnalyticsRepository for SqlxAnalyticsRepository {
    async fn save_report(&self, report: &DeploymentAnalyticsReport) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(report).unwrap();
        sqlx::query("INSERT INTO analytics_reports (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $2")
            .bind(report.report_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_reports(&self) -> Result<Vec<DeploymentAnalyticsReport>, RepositoryError> {
        let rows = sqlx::query_as::<_, Row>("SELECT id, data FROM analytics_reports")
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
    async fn get_strategy_performance(&self) -> Result<Vec<StrategyPerformance>, RepositoryError> {
        Ok(vec![])
    }
    async fn get_satellite_reliability(&self) -> Result<Vec<SatelliteReliability>, RepositoryError> {
        Ok(vec![])
    }
}
