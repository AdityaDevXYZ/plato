use async_trait::async_trait;
use sqlx::PgPool;
use identity::{user::{User, UserStatus}, repository::{UserRepository, RepositoryError}};
use types::{UserId, Email};
use uuid::Uuid;

pub struct SqlxUserRepository { pool: PgPool }
impl SqlxUserRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { id: Uuid, email: String, password_hash: String, password_updated_at: time::OffsetDateTime, email_verified: bool, status: String }

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        let id: Uuid = user.id().into();
        let email = user.email().as_str();
        let status = format!("{:?}", user.status());
        let p_hash = user.password_hash();
        let p_upd = user.password_updated_at();
        let p_ver = user.email_verified();
        sqlx::query("INSERT INTO users (id, email, password_hash, password_updated_at, email_verified, status) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO UPDATE SET email = $2, status = $6")
            .bind(id)
            .bind(email)
            .bind(p_hash)
            .bind(p_upd)
            .bind(p_ver)
            .bind(status)
            .execute(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
    
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError> {
        let email_str = email.as_str();
        let row = sqlx::query_as::<_, Row>("SELECT id, email, password_hash, password_updated_at, email_verified, status FROM users WHERE email = $1")
            .bind(email_str)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row {
            let email = Email::new(r.email).map_err(|_| RepositoryError::Database("Bad email".into()))?;
            let status = if r.status == "Active" { UserStatus::Active } else { UserStatus::Inactive };
            Ok(Some(User::reconstruct(r.id.into(), email, r.password_hash, r.password_updated_at, r.email_verified, status)))
        } else { Ok(None) }
    }
    
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, RepositoryError> {
        let uuid: Uuid = id.into();
        let row = sqlx::query_as::<_, Row>("SELECT id, email, password_hash, password_updated_at, email_verified, status FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.pool).await.map_err(|e| RepositoryError::Database(e.to_string()))?;
        if let Some(r) = row {
            let email = Email::new(r.email).map_err(|_| RepositoryError::Database("Bad email".into()))?;
            let status = if r.status == "Active" { UserStatus::Active } else { UserStatus::Inactive };
            Ok(Some(User::reconstruct(r.id.into(), email, r.password_hash, r.password_updated_at, r.email_verified, status)))
        } else { Ok(None) }
    }
}
