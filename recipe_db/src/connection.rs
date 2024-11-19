use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        Ok(Database {
            pool: PgPool::connect(database_url).await?,
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
