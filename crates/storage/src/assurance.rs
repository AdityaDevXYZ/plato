use async_trait::async_trait;
use sqlx::PgPool;
use assurance::{model::AssuranceReport, repository::{AssuranceReportRepository, RepositoryError}};
use types::ReleaseId;
use uuid::Uuid;

pub struct SqlxAssuranceRepository { pool: PgPool }
impl SqlxAssuranceRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { release_id: Uuid, data: serde_json::Value }

#[async_trait]
impl AssuranceReportRepository for SqlxAssuranceRepository {
    async fn save(&self, report: &AssuranceReport) -> Result<(), RepositoryError> {
        let rel_id: Uuid = report.release_id.into();
        let data = serde_json::to_value(report).unwrap();
        sqlx::query("INSERT INTO assurance_reports (release_id, data) VALUES ($1, $2) ON CONFLICT (release_id) DO UPDATE SET data = $2")
            .bind(rel_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn find_by_release_id(&self, release_id: ReleaseId) -> Result<Option<AssuranceReport>, RepositoryError> {
        let rel_id: Uuid = release_id.into();
        let row = sqlx::query_as::<_, Row>("SELECT release_id, data FROM assurance_reports WHERE release_id = $1")
            .bind(rel_id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row {
            Ok(Some(serde_json::from_value(r.data).unwrap()))
        } else { Ok(None) }
    }
}
