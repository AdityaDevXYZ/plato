use async_trait::async_trait;
use sqlx::PgPool;
use project::{model::{Project, ProjectStatus, ProjectName}, repository::{ProjectRepository, RepositoryError}};
use types::ProjectId;
use uuid::Uuid;

pub struct SqlxProjectRepository {
    pool: PgPool,
}

impl SqlxProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct Row {
    id: Uuid,
    org_id: Uuid,
    name: String,
    status: String,
}

#[async_trait]
impl ProjectRepository for SqlxProjectRepository {
    async fn save(&self, proj: &Project) -> Result<(), RepositoryError> {
        let id: Uuid = proj.id().into();
        let org_id: Uuid = proj.org_id().into();
        let name: &str = proj.name().as_str();
        let status = format!("{:?}", proj.status());

        sqlx::query("INSERT INTO projects (id, org_id, name, status) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET org_id = $2, name = $3, status = $4")
            .bind(id)
            .bind(org_id)
            .bind(name)
            .bind(status)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepositoryError> {
        let uuid: Uuid = id.into();
        let row = sqlx::query_as::<_, Row>("SELECT id, org_id, name, status FROM projects WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if let Some(r) = row {
            let status = match r.status.as_str() {
                "Archived" => ProjectStatus::Archived,
                _ => ProjectStatus::Active,
            };
            let name = ProjectName::new(r.name)
                .map_err(|e| RepositoryError::Database(format!("{:?}", e)))?;
            Ok(Some(Project::reconstruct(r.id.into(), name, r.org_id.into(), status)))
        } else {
            Ok(None)
        }
    }
}
