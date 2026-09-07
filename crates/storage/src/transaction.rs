use sqlx::{Postgres, Transaction};

/// Conceptual UnitOfWork abstracting SQLx implementation.
pub struct AppTransaction<'a> {
    pub(crate) inner: Transaction<'a, Postgres>,
}

impl<'a> AppTransaction<'a> {
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.inner.commit().await
    }
    
    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.inner.rollback().await
    }
}
