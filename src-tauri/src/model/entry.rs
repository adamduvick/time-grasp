use sqlx::types::Uuid;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use thiserror::Error;

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct Entry {
    pub id: i64,
    pub global_id: Uuid,
    pub payee: String,
    pub start_time: String,       // RFC3339
    pub end_time: Option<String>, // RFC3339
    pub duration_ms: Option<i64>,
    pub memo: Option<String>,
    pub category_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub version: i64,
}

pub trait Queryable {
    const ENTITY: &'static str;
    const FIELD_NAMES: &'static str;
    fn select(&self) -> String;
    fn returing(&self) -> String;
}

impl Queryable for Entry {
    const ENTITY: &'static str = "entries";
    const FIELD_NAMES: &'static str = r#"
        id, 
        global_id, 
        payee, 
        start_time, 
        end_time, 
        duration_ms, 
        memo, 
        category_id, 
        created_at, 
        updated_at, 
        deleted_at, 
        version"#;

    fn select(self: &Self) -> String {
        format!("SELECT {} FROM {}", Self::FIELD_NAMES, Self::ENTITY)
    }

    fn returing(self: &Self) -> String {
        format!("RETURNING {}", Self::FIELD_NAMES)
    }
}

#[derive(Debug, Clone)]
pub struct NewEntry {
    pub global_id: Uuid,
    pub payee: String,
    pub start_time: String,       // RFC3339
    pub end_time: Option<String>, // RFC3339
    pub memo: Option<String>,
    pub category_id: Option<i64>, // None → DEFAULT (Uncategorized)
}

#[derive(Debug, Clone, Default)]
pub struct EntryFilter {
    pub active_only: bool, // deleted_at IS NULL
    pub category_id: Option<i64>,
    pub start_from: Option<String>, // RFC3339 inclusive
    pub start_to: Option<String>,   // RFC3339 inclusive
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("not found")]
    NotFound,
    #[error("stale write (version mismatch)")]
    StaleWrite,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub struct EntryRepo;

impl EntryRepo {
    // CREATE
    pub async fn create(pool: &SqlitePool, n: NewEntry) -> Result<Entry, RepoError> {
        let rec = if let Some(cat) = n.category_id {
            sqlx::query_as::<_, Entry>(
                r#"
                INSERT INTO entries (global_id, payee, start_time, end_time, memo, category_id)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                RETURNING id, global_id, payee, start_time, end_time, duration_ms, memo,
                          category_id, created_at, updated_at, deleted_at, version
                "#,
            )
            .bind(n.global_id)
            .bind(n.payee)
            .bind(n.start_time)
            .bind(n.end_time)
            .bind(n.memo)
            .bind(cat)
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_as::<_, Entry>(
                r#"
                INSERT INTO entries (global_id, payee, start_time, end_time, memo)
                VALUES (?1, ?2, ?3, ?4, ?5)
                RETURNING id, global_id, payee, start_time, end_time, duration_ms, memo,
                          category_id, created_at, updated_at, deleted_at, version
                "#,
            )
            .bind(n.global_id)
            .bind(n.payee)
            .bind(n.start_time)
            .bind(n.end_time)
            .bind(n.memo)
            .fetch_one(pool)
            .await?
        };
        Ok(rec)
    }

    // READ (by id)
    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Entry, RepoError> {
        let rec = sqlx::query_as::<_, Entry>(
            r#"
            SELECT id, global_id, payee, start_time, end_time, duration_ms, memo,
                   category_id, created_at, updated_at, deleted_at, version
            FROM entries WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        rec.ok_or(RepoError::NotFound)
    }

    // READ (by global_id)
    pub async fn get_by_global(pool: &SqlitePool, gid: Uuid) -> Result<Entry, RepoError> {
        let rec = sqlx::query_as::<_, Entry>(
            r#"
            SELECT id, global_id, payee, start_time, end_time, duration_ms, memo,
                   category_id, created_at, updated_at, deleted_at, version
            FROM entries WHERE global_id = ?1
            "#,
        )
        .bind(gid)
        .fetch_optional(pool)
        .await?;
        rec.ok_or(RepoError::NotFound)
    }

    // LIST with filters (active, category, start range, pagination)
    pub async fn list(pool: &SqlitePool, f: EntryFilter) -> Result<Vec<Entry>, RepoError> {
        let mut qb = QueryBuilder::<Sqlite>::new(
            "SELECT id, global_id, payee, start_time, end_time, duration_ms, memo, \
                    category_id, created_at, updated_at, deleted_at, version \
             FROM entries",
        );

        // Build WHERE clause safely (one WHERE, AND between conditions)
        let mut any = false;
        if f.active_only {
            if !any {
                qb.push(" WHERE ");
                any = true;
            } else {
                qb.push(" AND ");
            }
            qb.push("deleted_at IS NULL");
        }
        if let Some(cat) = f.category_id {
            if !any {
                qb.push(" WHERE ");
                any = true;
            } else {
                qb.push(" AND ");
            }
            qb.push("category_id = ").push_bind(cat);
        }
        if let Some(s) = f.start_from {
            if !any {
                qb.push(" WHERE ");
                any = true;
            } else {
                qb.push(" AND ");
            }
            qb.push("julianday(start_time) >= julianday(")
                .push_bind(s)
                .push(")");
        }
        if let Some(s) = f.start_to {
            if !any {
                qb.push(" WHERE ");
                any = true;
            } else {
                qb.push(" AND ");
            }
            qb.push("julianday(start_time) <= julianday(")
                .push_bind(s)
                .push(")");
        }

        qb.push(" ORDER BY start_time DESC");

        if let Some(lim) = f.limit {
            qb.push(" LIMIT ").push_bind(lim);
        }
        if let Some(off) = f.offset {
            qb.push(" OFFSET ").push_bind(off);
        }

        let recs = qb.build_query_as::<Entry>().fetch_all(pool).await?;
        Ok(recs)
    }

    // UPDATE (optimistic locking): pass current version; trigger will bump it.
    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        version: i64,
        payee: Option<String>,
        start_time: Option<String>,
        end_time: Option<Option<String>>, // Some(None) clears end_time
        memo: Option<Option<String>>,
        category_id: Option<i64>,
    ) -> Result<Entry, RepoError> {
        // Build SET clause pieces
        let mut sets: Vec<&str> = Vec::new();
        if payee.is_some() {
            sets.push("payee = ?");
        }
        if start_time.is_some() {
            sets.push("start_time = ?");
        }
        if end_time.is_some() {
            sets.push("end_time = ?");
        } // binds Option<String>
        if memo.is_some() {
            sets.push("memo = ?");
        } // binds Option<String>
        if category_id.is_some() {
            sets.push("category_id = ?");
        }
        // Always include a deterministic assignment so SET is never empty
        sets.push("version = version");

        let set_clause = sets.join(", ");
        let sql = format!(
            "UPDATE entries SET {} WHERE id = ? AND version = ? \
         RETURNING id, global_id, payee, start_time, end_time, duration_ms, memo, \
                   category_id, created_at, updated_at, deleted_at, version",
            set_clause
        );

        let mut q = sqlx::query_as::<_, Entry>(&sql);
        if let Some(v) = payee {
            q = q.bind(v);
        }
        if let Some(v) = start_time {
            q = q.bind(v);
        }
        if let Some(v) = end_time {
            q = q.bind(v);
        } // Option<String>
        if let Some(v) = memo {
            q = q.bind(v);
        } // Option<String>
        if let Some(v) = category_id {
            q = q.bind(v);
        }

        let rec = q.bind(id).bind(version).fetch_optional(pool).await?;
        rec.ok_or(RepoError::StaleWrite)
    }

    // SOFT DELETE (optimistic)
    pub async fn soft_delete(pool: &SqlitePool, id: i64, version: i64) -> Result<(), RepoError> {
        let rows = sqlx::query!(
            r#"
            UPDATE entries
               SET deleted_at = strftime('%Y-%m-%dT%H:%M:%fZ','now'),
                   version    = version   -- keep same to trigger bump
             WHERE id = ?1 AND version = ?2
            "#,
            id,
            version
        )
        .execute(pool)
        .await?
        .rows_affected();

        if rows == 0 {
            return Err(RepoError::StaleWrite);
        }
        Ok(())
    }

    // UNDELETE (optimistic)
    pub async fn undelete(pool: &SqlitePool, id: i64, version: i64) -> Result<Entry, RepoError> {
        let rec = sqlx::query_as::<_, Entry>(
            r#"
            UPDATE entries
               SET deleted_at = NULL,
                   version    = version
             WHERE id = ?1 AND version = ?2
             RETURNING id, global_id, payee, start_time, end_time, duration_ms, memo,
                       category_id, created_at, updated_at, deleted_at, version
            "#,
        )
        .bind(id)
        .bind(version)
        .fetch_optional(pool)
        .await?;
        rec.ok_or(RepoError::StaleWrite)
    }

    // HARD DELETE (rare; usually prefer soft-delete)
    pub async fn hard_delete(pool: &SqlitePool, id: i64) -> Result<(), RepoError> {
        let rows = sqlx::query!("DELETE FROM entries WHERE id = ?1", id)
            .execute(pool)
            .await?
            .rows_affected();
        if rows == 0 {
            return Err(RepoError::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{sqlite::SqlitePoolOptions, Pool};
    use uuid::Uuid as StdUuid;

    async fn setup_pool() -> SqlitePool {
        // Single-connection pool so we can wrap tests in a single tx and roll it back.
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:?cache=shared")
            .await
            .expect("connect in-memory sqlite");

        // Run migrations (path is crate-relative; adjust if needed).
        // Assumes `src-tauri/migrations` directory exists for this crate.
        static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
        MIGRATOR.run(&pool).await.expect("migrations");

        pool
    }

    /// Run the provided future inside a BEGIN/ROLLBACK on the *single* pooled connection.
    async fn with_tx<F, Fut, T>(pool: &SqlitePool, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        // Start tx on the only connection.
        sqlx::query("BEGIN IMMEDIATE")
            .execute(pool)
            .await
            .expect("begin tx");
        let out = f().await;
        // Always rollback to discard changes.
        sqlx::query("ROLLBACK")
            .execute(pool)
            .await
            .expect("rollback tx");
        out
    }

    fn rfc3339_now() -> String {
        // Cheap RFC3339 with UTC using `time` crate already in deps
        let now = time::OffsetDateTime::now_utc();
        now.format(&time::format_description::well_known::Rfc3339)
            .expect("fmt")
    }

    fn sample_new_entry() -> NewEntry {
        NewEntry {
            global_id: Uuid::from_bytes(*StdUuid::new_v4().as_bytes()),
            payee: "Test Payee".to_string(),
            start_time: rfc3339_now(),
            end_time: None,
            memo: Some("hello".into()),
            category_id: None, // use DEFAULT (Uncategorized)
        }
    }

    #[tokio::test]
    async fn create_and_get_roundtrip() {
        let pool = setup_pool().await;
        with_tx(&pool, || async {
            let created = EntryRepo::create(&pool, sample_new_entry())
                .await
                .expect("create");
            assert!(created.id > 0);
            assert_eq!(created.payee, "Test Payee");
            assert!(created.duration_ms.is_none()); // no end_time yet
            assert_eq!(created.deleted_at, None);

            let by_id = EntryRepo::get(&pool, created.id).await.expect("get by id");
            assert_eq!(by_id.global_id, created.global_id);

            let by_gid = EntryRepo::get_by_global(&pool, created.global_id)
                .await
                .expect("get by gid");
            assert_eq!(by_gid.id, created.id);
        })
        .await;
    }

    #[tokio::test]
    async fn list_filters_work() {
        let pool = setup_pool().await;
        with_tx(&pool, || async {
            // insert 2 entries different start_time and categories
            let mut e1 = sample_new_entry();
            e1.payee = "A".into();
            let created1 = EntryRepo::create(&pool, e1).await.unwrap();

            let mut e2 = sample_new_entry();
            e2.payee = "B".into();
            e2.category_id = Some(1); // explicit uncategorized
            let created2 = EntryRepo::create(&pool, e2).await.unwrap();

            // Active only
            let active = EntryRepo::list(
                &pool,
                EntryFilter {
                    active_only: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert!(active.iter().any(|e| e.id == created1.id));
            assert!(active.iter().any(|e| e.id == created2.id));

            // Filter by category_id
            let only_uncat = EntryRepo::list(
                &pool,
                EntryFilter {
                    active_only: true,
                    category_id: Some(1),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert!(only_uncat.iter().all(|e| e.category_id == 1));

            // Pagination: limit 1
            let page = EntryRepo::list(
                &pool,
                EntryFilter {
                    limit: Some(1),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(page.len(), 1);
        })
        .await;
    }

    #[tokio::test]
    async fn update_and_stale_write() {
        let pool = setup_pool().await;
        with_tx(&pool, || async {
            let created = EntryRepo::create(&pool, sample_new_entry()).await.unwrap();

            // Successful update
            let updated = EntryRepo::update(
                &pool,
                created.id,
                created.version,
                Some("New Payee".into()),
                None,
                Some(Some(rfc3339_now())),
                Some(None), // clear memo
                Some(1),
            )
            .await
            .unwrap();
            assert_eq!(updated.payee, "New Payee");
            assert!(updated.memo.is_none());
            assert!(updated.duration_ms.is_some()); // end_time set should compute duration

            // Stale write: reuse old version
            let stale = EntryRepo::update(
                &pool,
                created.id,
                created.version, // old version
                Some("Another".into()),
                None,
                None,
                None,
                None,
            )
            .await
            .err()
            .expect("stale write");
            matches!(stale, RepoError::StaleWrite);
        })
        .await;
    }

    #[tokio::test]
    async fn soft_delete_and_undelete() {
        let pool = setup_pool().await;
        with_tx(&pool, || async {
            let created = EntryRepo::create(&pool, sample_new_entry()).await.unwrap();

            // soft delete
            EntryRepo::soft_delete(&pool, created.id, created.version)
                .await
                .expect("soft delete");

            // verify filtered out by active_only
            let active = EntryRepo::list(
                &pool,
                EntryFilter {
                    active_only: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert!(!active.iter().any(|e| e.id == created.id));

            // fetch current to get new version (deleted updated by trigger)
            let deleted_row = EntryRepo::get(&pool, created.id).await.unwrap();
            assert!(deleted_row.deleted_at.is_some());

            // undelete
            let restored = EntryRepo::undelete(&pool, created.id, deleted_row.version)
                .await
                .expect("undelete");
            assert!(restored.deleted_at.is_none());
        })
        .await;
    }

    #[tokio::test]
    async fn hard_delete_removes_row() {
        let pool = setup_pool().await;
        with_tx(&pool, || async {
            let created = EntryRepo::create(&pool, sample_new_entry()).await.unwrap();
            EntryRepo::hard_delete(&pool, created.id).await.unwrap();
            let err = EntryRepo::get(&pool, created.id).await.err().unwrap();
            matches!(err, RepoError::NotFound);
        })
        .await;
    }
}
