use async_trait::async_trait;
use sqlx::PgPool;
use risk::{model::RiskAssessmentReport, repository::{RiskRepository, RepositoryError}};
use uuid::Uuid;

pub struct SqlxRiskRepository { pool: PgPool }
impl SqlxRiskRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, plan_id: Uuid, data: serde_json::Value }

#[async_trait]
impl RiskRepository for SqlxRiskRepository {
    async fn save_report(&self, report: &RiskAssessmentReport) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(report).unwrap();
        sqlx::query("INSERT INTO risk_reports (id, plan_id, data) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET data = $3")
            .bind(report.report_id)
            .bind(report.plan_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_report(&self, id: Uuid) -> Result<Option<RiskAssessmentReport>, RepositoryError> {
        let row = sqlx::query_as::<_, Row>("SELECT id, plan_id, data FROM risk_reports WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row { Ok(Some(serde_json::from_value(r.data).unwrap())) } else { Ok(None) }
    }
    async fn get_reports_by_plan(&self, plan_id: Uuid) -> Result<Vec<RiskAssessmentReport>, RepositoryError> {
        let rows = sqlx::query_as::<_, Row>("SELECT id, plan_id, data FROM risk_reports WHERE plan_id = $1")
            .bind(plan_id)
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
}
