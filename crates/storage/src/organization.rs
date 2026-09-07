use async_trait::async_trait;
use sqlx::PgPool;
use organization::{model::{Organization, OrganizationStatus, OrganizationName}, repository::{OrganizationRepository, RepositoryError}};
use types::OrganizationId;
use uuid::Uuid;

pub struct SqlxOrganizationRepository {
    pool: PgPool,
}

impl SqlxOrganizationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct Row {
    id: Uuid,
    name: String,
    owner_id: Uuid,
    status: String,
}

#[async_trait]
impl OrganizationRepository for SqlxOrganizationRepository {
    async fn save(&self, org: &Organization) -> Result<(), RepositoryError> {
        let id: Uuid = org.id().into();
        let name: &str = org.name().as_str();
        let owner_id: Uuid = org.owner_id().into();
        let status = format!("{:?}", org.status());

        sqlx::query("INSERT INTO organizations (id, name, owner_id, status) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET name = $2, owner_id = $3, status = $4")
            .bind(id)
            .bind(name)
            .bind(owner_id)
            .bind(status)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: OrganizationId) -> Result<Option<Organization>, RepositoryError> {
        let uuid: Uuid = id.into();
        let row = sqlx::query_as::<_, Row>("SELECT id, name, owner_id, status FROM organizations WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if let Some(r) = row {
            let status = match r.status.as_str() {
                "Suspended" => OrganizationStatus::Suspended,
                _ => OrganizationStatus::Active,
            };
            let name = OrganizationName::new(r.name)
                .map_err(|e| RepositoryError::Database(format!("{:?}", e)))?;
            Ok(Some(Organization::reconstruct(r.id.into(), name, r.owner_id.into(), status)))
        } else {
            Ok(None)
        }
    }
}
