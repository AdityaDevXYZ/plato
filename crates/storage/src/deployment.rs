use async_trait::async_trait;
use sqlx::PgPool;
use deployment::{model::DeploymentSession, repository::{DeploymentSessionRepository, RepositoryError}};
use uuid::Uuid;

pub struct SqlxDeploymentRepository { pool: PgPool }
impl SqlxDeploymentRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, data: serde_json::Value }

#[async_trait]
impl DeploymentSessionRepository for SqlxDeploymentRepository {
    async fn save(&self, session: &DeploymentSession) -> Result<(), RepositoryError> {
        let id = session.session_id;
        let data = serde_json::to_value(session).unwrap();
        sqlx::query("INSERT INTO deployment_sessions (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $2")
            .bind(id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn find_by_id(&self, session_id: Uuid) -> Result<Option<DeploymentSession>, RepositoryError> {
        let row = sqlx::query_as::<_, Row>("SELECT id, data FROM deployment_sessions WHERE id = $1")
            .bind(session_id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row {
            Ok(Some(serde_json::from_value(r.data).unwrap()))
        } else { Ok(None) }
    }
}
