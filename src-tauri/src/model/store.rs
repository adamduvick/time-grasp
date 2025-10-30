use sqlx::{
    sqlite::{self, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};

pub struct Store(sqlite::SqlitePool);

impl Store {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Path to your SQLite database file.
        // ":memory:" for in-memory; "time_grasp.db" for persistent file.
        let opts = SqliteConnectOptions::new()
            .filename(database_url)
            .create_if_missing(true)
            .foreign_keys(true) // PRAGMA foreign_keys=ON for each conn
            .journal_mode(SqliteJournalMode::Wal); // optional: better concurrency

        let pool: SqlitePool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await?;

        // Apply migrations. Assumes your .sql files live in a `migrations/` folder.
        // Example: `migrations/0001_init.sql`
        //
        // The `sqlx::migrate!()` macro embeds migration metadata at compile time.
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Store(pool))
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.0
    }
}
