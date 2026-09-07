use async_trait::async_trait;
use sqlx::PgPool;
use coordination::{model::CoordinationSession, repository::{CoordinationRepository, RepositoryError}};
use uuid::Uuid;

pub struct SqlxCoordinationRepository { pool: PgPool }
impl SqlxCoordinationRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, plan_id: Uuid, data: serde_json::Value }

#[async_trait]
impl CoordinationRepository for SqlxCoordinationRepository {
    async fn save_session(&self, session: &CoordinationSession) -> Result<(), RepositoryError> {
        let data = serde_json::to_value(session).unwrap();
        sqlx::query("INSERT INTO coordination_sessions (id, plan_id, data) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET data = $3")
            .bind(session.session_id)
            .bind(session.plan_id)
            .bind(data)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    async fn get_session(&self, id: Uuid) -> Result<Option<CoordinationSession>, RepositoryError> {
        let row = sqlx::query_as::<_, Row>("SELECT id, plan_id, data FROM coordination_sessions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row { Ok(Some(serde_json::from_value(r.data).unwrap())) } else { Ok(None) }
    }
    async fn get_sessions_by_plan(&self, plan_id: Uuid) -> Result<Vec<CoordinationSession>, RepositoryError> {
        let rows = sqlx::query_as::<_, Row>("SELECT id, plan_id, data FROM coordination_sessions WHERE plan_id = $1")
            .bind(plan_id)
            .fetch_all(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows.into_iter().map(|r| serde_json::from_value(r.data).unwrap()).collect())
    }
}
