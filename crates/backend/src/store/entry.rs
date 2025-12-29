//! Store adapter for the `entry` table.
//!
//! Implements the store traits for entries using `sqlx`. Provides helpers
//! and tests to validate correct behavior of create/read/update/delete
//! operations and filter application.
use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::store::*;
use model::*;

#[async_trait]
impl Creatable<C_Entry> for C_Entry {
    async fn create(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        sqlx::query(
            "INSERT INTO entry (
                id,
                name, 
                note,
                category_id,
                start_time,
                end_time
            ) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(&entity.name)
        .bind(&entity.note)
        .bind(&entity.category_id)
        .bind(&entity.start_time)
        .bind(&entity.end_time)
        .execute(pool)
        .await?;

        Ok(entity.id)
    }
}

#[async_trait]
impl Readable<R_Entry> for R_Entry {
    type Filter = EntryFilter;

    const BASE_SELECT: &'static str = r#"
        SELECT 
            id,
            name, 
            note, 
            category_id, 
            start_time, 
            end_time, 
            duration, 
            version, 
            created_at, 
            updated_at,
            deleted_at,
            deleted_by_user,
            deleted_by_device,
            tombstone_reason
        FROM entry
    "#;

    async fn read(pool: &SqlitePool, id: Uuid) -> Result<Self> {
        let mut qb = QueryBuilder::new(Self::BASE_SELECT);
        let filter = Self::Filter::new().id(Some(id));
        filter.apply(&mut qb);
        let entity = qb.build_query_as().fetch_one(pool).await?;

        Ok(entity)
    }

    async fn list(pool: &SqlitePool, filter: Self::Filter) -> Result<Vec<Self>> {
        let mut qb = QueryBuilder::new(Self::BASE_SELECT);
        filter.apply(&mut qb);
        let entities = qb.build_query_as().fetch_all(pool).await?;

        Ok(entities)
    }
}

impl Filterable for EntryFilter {
    fn apply(&self, qb: &mut QueryBuilder<Sqlite>) {
        qb.push(" WHERE 1=1");

        let Self { id } = self.clone();

        if let Some(id) = id {
            qb.push(" AND id = ").push_bind(id);
        }
    }
}

#[async_trait]
impl Updatable<U_Entry> for U_Entry {
    async fn update(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        let Self {
            id,
            name,
            note,
            category_id,
            start_time,
            end_time,
        } = entity;

        let name_flag = name.is_some();
        let note_flag = note.is_some();
        let category_flag = category_id.is_some();
        let start_flag = start_time.is_some();
        let end_flag = end_time.is_some();

        // No fields requested to change → skip hitting the DB
        if !name_flag && !note_flag && !category_flag && !start_flag && !end_flag {
            return Ok(id);
        }

        let name_value = name;
        let note_value = note.flatten();
        let category_value = category_id;
        let start_value = start_time;
        let end_value = end_time.flatten();

        sqlx::query(
            "UPDATE entry SET
                name = CASE WHEN ? THEN ? ELSE name END,
                note = CASE WHEN ? THEN ? ELSE note END,
                category_id = CASE WHEN ? THEN ? ELSE category_id END,
                start_time = CASE WHEN ? THEN ? ELSE start_time END,
                end_time = CASE WHEN ? THEN ? ELSE end_time END
            WHERE id = ?",
        )
        .bind(name_flag)
        .bind(name_value)
        .bind(note_flag)
        .bind(note_value)
        .bind(category_flag)
        .bind(category_value)
        .bind(start_flag)
        .bind(start_value)
        .bind(end_flag)
        .bind(end_value)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(id)
    }
}

#[async_trait]
impl Deletable<D_Entry> for D_Entry {
    async fn delete(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        sqlx::query(
            "UPDATE entry SET 
                deleted_by_user = ?, 
                deleted_by_device = ?, 
                tombstone_reason = ? 
            WHERE id = ?",
        )
        .bind(Uuid::new_v4())
        .bind(Uuid::new_v4())
        .bind(entity.tombstone_reason)
        .bind(entity.id)
        .execute(pool)
        .await?;

        Ok(entity.id)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use model::{Category, CategoryGroup, DurationMillis, EpochMillis};

    pub async fn create_and_read(
        pool: &SqlitePool,
        id: Option<Uuid>,
        name: Option<String>,
        note: Option<String>,
        category_id: Uuid,
        start_time: Option<EpochMillis>,
        end_time: Option<EpochMillis>,
    ) -> Result<R_Entry> {
        let entity_id = id.unwrap_or(Uuid::new_v4());
        let entity = C_Entry {
            id: entity_id.clone(),
            name: name.unwrap_or("Test R_Entry".to_string()),
            note: note.map(|s| s.to_string()),
            category_id: category_id,
            start_time: start_time.unwrap_or(EpochMillis::now()),
            end_time: end_time,
        };
        C_Entry::create(pool, entity).await?;
        R_Entry::read(&pool, entity_id).await
    }

    pub async fn update_and_read(
        pool: &SqlitePool,
        id: Uuid,
        name: Option<String>,
        note: Option<Option<String>>,
        category_id: Option<Uuid>,
        start_time: Option<EpochMillis>,
        end_time: Option<Option<EpochMillis>>,
    ) -> Result<R_Entry> {
        let params = U_Entry {
            id,
            name,
            note,
            category_id,
            start_time,
            end_time,
        };
        U_Entry::update(pool, params).await?;
        R_Entry::read(&pool, id).await
    }

    pub async fn delete_and_read(
        pool: &SqlitePool,
        id: Uuid,
        reason: Option<String>,
    ) -> Result<R_Entry> {
        let params = D_Entry {
            id,
            tombstone_reason: reason.unwrap_or("User request".to_string()),
        };
        D_Entry::delete(pool, params).await?;
        R_Entry::read(&pool, id).await
    }

    async fn create_chain(pool: &SqlitePool) -> Result<(CategoryGroup, Category, R_Entry)> {
        let group = create_and_read_group(&pool, None, None, None).await?;
        let category = create_and_read_category(&pool, None, None, None, group.id).await?;
        let entry = create_and_read(&pool, None, None, None, category.id, None, None).await?;
        Ok((group, category, entry))
    }

    #[sqlx::test]
    async fn test_create(pool: SqlitePool) {
        let name = "Test R_Entry".to_string();
        let note = None;

        let (_group, category, entity) = create_chain(&pool).await.expect("Failed to create chain");

        assert_eq!(entity.name, name);
        assert_eq!(entity.note, note);
        assert_eq!(entity.category_id, category.id);
        assert_eq!(entity.version, 1);
        assert!((EpochMillis::now() - entity.created_at).0 <= 10); // within 10 milliseconds
        assert_eq!(entity.created_at, entity.updated_at);
        assert!(entity.deleted_at.is_none());
        assert!(entity.deleted_by_user.is_none());
        assert!(entity.deleted_by_device.is_none());
        assert!(entity.tombstone_reason.is_none());
    }

    #[sqlx::test]
    async fn test_create_with_fake_category_fails(pool: SqlitePool) {
        let fake_category_id = Uuid::new_v4();
        let entity = create_and_read(&pool, None, None, None, fake_category_id, None, None).await;

        assert!(entity.is_err());
    }

    #[sqlx::test]
    async fn test_create_dupe_id_fails(pool: SqlitePool) {
        let (_group, category, entity1) =
            create_chain(&pool).await.expect("Failed to create chain");
        let entity2 =
            create_and_read(&pool, Some(entity1.id), None, None, category.id, None, None).await;

        assert!(entity2.is_err());
    }

    #[sqlx::test]
    async fn test_update(pool: SqlitePool) {
        let name = "updated name".to_string();
        let note = "updated note".to_string();
        let (_group, _category, entity) =
            create_chain(&pool).await.expect("Failed to create chain");

        assert_ne!(entity.name, name);
        assert_ne!(entity.note, Some(note.clone()));

        let updated = update_and_read(
            &pool,
            entity.id,
            Some(name.clone()),
            Some(Some(note.clone())),
            None,
            None,
            None,
        )
        .await
        .expect("Failed to update and read entry");

        assert_eq!(updated.name, name);
        assert_eq!(updated.note, Some(note));
        assert_eq!(updated.version, entity.version + 1);
        assert!(updated.updated_at >= entity.updated_at);
    }

    #[sqlx::test]
    async fn test_start_time_greater_than_end_time_fails(pool: SqlitePool) {
        let start_time = EpochMillis::now();
        let end_time = EpochMillis(start_time.0 - 1000); // 1 second before start_time
        let (_group, _category, entity) =
            create_chain(&pool).await.expect("Failed to create chain");

        let updated = update_and_read(
            &pool,
            entity.id,
            None,
            None,
            None,
            Some(start_time),
            Some(Some(end_time)),
        )
        .await;

        assert!(updated.is_err());
    }

    #[sqlx::test]
    async fn test_duration_read_back(pool: SqlitePool) {
        let start_time = EpochMillis::now();
        let end_time = EpochMillis(start_time.0 + 5000); // 5 seconds after start_time
        let (_group, _category, entity) =
            create_chain(&pool).await.expect("Failed to create chain");

        let updated = update_and_read(
            &pool,
            entity.id,
            None,
            None,
            None,
            Some(start_time),
            Some(Some(end_time)),
        )
        .await
        .expect("Failed to update and read entry");

        assert_eq!(updated.start_time, start_time);
        assert_eq!(updated.end_time, Some(end_time));
        assert_eq!(updated.duration, Some(DurationMillis(5000)));
    }

    #[sqlx::test]
    async fn test_empty_update_does_nothing(pool: SqlitePool) {
        let (_group, _category, entity) =
            create_chain(&pool).await.expect("Failed to create chain");
        let updated = update_and_read(&pool, entity.id, None, None, None, None, None)
            .await
            .expect("Failed to update and read entry group");
        assert_eq!(updated, entity);
    }

    #[sqlx::test]
    async fn test_delete(pool: SqlitePool) {
        let reason = "No longer needed".to_string();
        let (_group, _category, entity) =
            create_chain(&pool).await.expect("Failed to create chain");

        assert!(entity.deleted_at.is_none());
        assert!(entity.deleted_by_user.is_none());
        assert!(entity.deleted_by_device.is_none());
        assert!(entity.tombstone_reason.is_none());

        let deleted = delete_and_read(&pool, entity.id, Some(reason.clone()))
            .await
            .expect("Failed to delete and read entry");

        assert_eq!(deleted.tombstone_reason, Some(reason));
        assert_eq!(deleted.version, entity.version + 1);
        assert_eq!(deleted.deleted_at, Some(deleted.updated_at));
        assert!(deleted.updated_at >= entity.updated_at);
        assert!(deleted.deleted_by_user.is_some());
        assert!(deleted.deleted_by_device.is_some());
    }

    #[sqlx::test]
    async fn test_update_after_delete_fails(_pool: SqlitePool) {
        // TODO: Implement this test once we have soft-delete enforcement in place
        assert!(true);
    }
}
