use anyhow::{Context, Result};
use sqlx::{
    sqlite::{self, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};

use crate::model::seed_for_dev;

pub struct Store(sqlite::SqlitePool);

impl Store {
    pub async fn new(database_url: String) -> Result<Self> {
        // Path to your SQLite database file.
        // ":memory:" for in-memory; "time_grasp.db" for persistent file.
        let opts = SqliteConnectOptions::new()
            .filename(database_url.clone())
            .create_if_missing(true)
            .foreign_keys(true) // PRAGMA foreign_keys=ON for each conn
            .journal_mode(SqliteJournalMode::Wal); // optional: better concurrency

        // Helpful diagnostics
        eprintln!("DB path: {}", database_url.clone());
        eprintln!("cwd: {}", std::env::current_dir()?.display());

        tracing::info!("{:?} connecting to database", database_url.clone());

        // let pool: SqlitePool = SqlitePoolOptions::new()
        //     .max_connections(5)
        //     .connect_with(opts)
        //     .await
        //     .context("model init failed")?;

        let pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

        // Apply migrations. Assumes your .sql files live in a `migrations/` folder.
        // Example: `migrations/0001_init.sql`
        //
        // The `sqlx::migrate!()` macro embeds migration metadata at compile time.
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("migrations failed")?;
        if let Err(e) = seed_for_dev::seed_dev_db(&pool).await {
            eprintln!("❌ Seeding skipped for the following reasons: {:?}", e);
        }

        Ok(Store(pool))
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.0
    }
}
