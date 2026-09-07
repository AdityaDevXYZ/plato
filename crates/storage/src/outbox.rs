use async_trait::async_trait;
use sqlx::PgPool;
use application::outbox_worker::OutboxRepository;
use uuid::Uuid;

pub struct SqlxOutboxRepository { pool: PgPool }
impl SqlxOutboxRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct OutboxRow { id: Uuid, event_type: String, payload: String }

#[derive(sqlx::FromRow)]
struct PayloadRow { payload: String }

#[async_trait]
impl OutboxRepository for SqlxOutboxRepository {
    async fn fetch_unpublished(&self, limit: i64) -> Result<Vec<(Uuid, String, String)>, String> {
        let rows = sqlx::query_as::<_, OutboxRow>("SELECT id, event_type, payload::text AS payload FROM outbox_messages WHERE status = 'PENDING' LIMIT $1")
            .bind(limit)
            .fetch_all(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(rows.into_iter().map(|r| (r.id, r.event_type, r.payload)).collect())
    }
    async fn mark_published(&self, id: Uuid) -> Result<(), String> {
        sqlx::query("UPDATE outbox_messages SET status = 'PUBLISHED' WHERE id = $1")
            .bind(id)
            .execute(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    async fn mark_failed(&self, id: Uuid, reason: &str) -> Result<(), String> {
        sqlx::query("UPDATE outbox_messages SET retry_count = retry_count + 1, last_error = $1 WHERE id = $2")
            .bind(reason)
            .bind(id)
            .execute(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    async fn move_to_dlq(&self, id: Uuid, reason: &str) -> Result<(), String> {
        let row = sqlx::query_as::<_, PayloadRow>("SELECT payload::text AS payload FROM outbox_messages WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool).await.map_err(|e| e.to_string())?;
        sqlx::query("INSERT INTO dlq_messages (id, payload, reason, created_at) VALUES ($1, $2, $3, NOW())")
            .bind(id)
            .bind(row.payload)
            .bind(reason)
            .execute(&self.pool).await.map_err(|e| e.to_string())?;
        sqlx::query("DELETE FROM outbox_messages WHERE id = $1")
            .bind(id)
            .execute(&self.pool).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
