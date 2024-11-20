use sqlx::{migrate::Migrator, PgPool};

pub struct Database {
    pool: PgPool,
}

pub static MIGRATOR: Migrator = sqlx::migrate!(); //defaults to ./migrations (on crate root)

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        Ok(Database {
            pool: PgPool::connect(database_url).await?,
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        MIGRATOR.run(&self.pool).await
    }
}
