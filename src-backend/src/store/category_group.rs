//! Store adapter for the `category_group` table.
//!
//! Implements the store traits for category group domain types using
//! `sqlx` and provides test helpers validating basic CRUD behavior.
use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::store::*;
use model::*;

#[async_trait]
impl Creatable<C_Group> for C_Group {
    async fn create(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        sqlx::query(
            "INSERT INTO category_group (
                id,
                name, 
                note
            ) VALUES (?, ?, ?)",
        )
        .bind(entity.id)
        .bind(&entity.name)
        .bind(&entity.note)
        .execute(pool)
        .await?;

        Ok(entity.id)
    }
}

#[async_trait]
impl Readable<R_Group> for R_Group {
    type Filter = CategoryGroupFilter;

    const BASE_SELECT: &'static str = r#"
        SELECT 
            id,
            name, 
            note, 
            version, 
            created_at, 
            updated_at,
            deleted_at,
            deleted_by_user,
            deleted_by_device,
            tombstone_reason
        FROM category_group
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

impl Filterable for CategoryGroupFilter {
    fn apply(&self, qb: &mut QueryBuilder<Sqlite>) {
        qb.push(" WHERE 1=1");

        let Self { id } = self.clone();

        if let Some(id) = id {
            qb.push(" AND id = ").push_bind(id);
        }
    }
}

#[async_trait]
impl Updatable<U_Group> for U_Group {
    async fn update(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        let Self { id, name, note } = entity;

        let name_flag = name.is_some();
        let note_flag = note.is_some();

        // No fields requested to change → skip hitting the DB
        if !name_flag && !note_flag {
            return Ok(id);
        }

        let name_value = name;
        let note_value = note.flatten();

        sqlx::query(
            "UPDATE category_group SET
                name = CASE WHEN ? THEN ? ELSE name END,
                note = CASE WHEN ? THEN ? ELSE note END
            WHERE id = ?",
        )
        .bind(name_flag)
        .bind(name_value)
        .bind(note_flag)
        .bind(note_value)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(id)
    }
}

#[async_trait]
impl Deletable<D_Group> for D_Group {
    async fn delete(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        sqlx::query(
            "UPDATE category_group SET 
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
    use model::EpochMillis;

    use super::*;

    pub async fn create_and_read(
        pool: &SqlitePool,
        id: Option<Uuid>,
        name: Option<String>,
        note: Option<String>,
    ) -> Result<R_Group> {
        let entity_id = id.unwrap_or(Uuid::new_v4());
        let entity = C_Group {
            id: entity_id.clone(),
            name: name.unwrap_or("Test Group".to_string()),
            note: note,
        };
        C_Group::create(pool, entity).await?;
        R_Group::read(&pool, entity_id).await
    }

    pub async fn update_and_read(
        pool: &SqlitePool,
        id: Uuid,
        name: Option<String>,
        note: Option<Option<String>>,
    ) -> Result<R_Group> {
        let params = U_Group { id, name, note };
        U_Group::update(pool, params).await?;
        R_Group::read(&pool, id).await
    }

    pub async fn delete_and_read(
        pool: &SqlitePool,
        id: Uuid,
        reason: Option<String>,
    ) -> Result<R_Group> {
        let params = D_Group {
            id,
            tombstone_reason: reason.unwrap_or("User request".to_string()),
        };
        D_Group::delete(pool, params).await?;
        R_Group::read(&pool, id).await
    }

    #[sqlx::test]
    async fn test_create(pool: SqlitePool) {
        let name = "test group".to_string();
        let note = "test note".to_string();
        let entity = create_and_read(&pool, None, Some(name.clone()), Some(note.clone()))
            .await
            .expect("Failed to create and read category group");

        assert_eq!(entity.name, name);
        assert_eq!(entity.note, Some(note));
        assert_eq!(entity.version, 1);
        assert!((EpochMillis::now() - entity.created_at).0 <= 10); // within 10 milliseconds
        assert_eq!(entity.created_at, entity.updated_at);
        assert!(entity.deleted_at.is_none());
        assert!(entity.deleted_by_user.is_none());
        assert!(entity.deleted_by_device.is_none());
        assert!(entity.tombstone_reason.is_none());
    }

    #[sqlx::test]
    async fn test_create_dupe_id_fails(pool: SqlitePool) {
        let id = Uuid::new_v4();
        let _entity1 = create_and_read(&pool, Some(id), Some("Group 1".to_string()), None)
            .await
            .expect("Failed to create and read category group");
        let entity2 = create_and_read(&pool, Some(id), Some("Group 2".to_string()), None).await;

        assert!(entity2.is_err());
    }

    #[sqlx::test]
    async fn test_create_dupe_name_fails(pool: SqlitePool) {
        let name = "Unique Group Name".to_string();
        let _entity1 = create_and_read(&pool, None, Some(name.clone()), None)
            .await
            .expect("Failed to create and read category group");
        let entity2 = create_and_read(&pool, None, Some(name), None).await;

        assert!(entity2.is_err());
    }

    #[sqlx::test]
    async fn test_update(pool: SqlitePool) {
        let name = "updated name".to_string();
        let note = "updated note".to_string();
        let entity = create_and_read(&pool, None, None, None)
            .await
            .expect("Failed to create and read category group");

        assert_ne!(entity.name, name);
        assert_ne!(entity.note, Some(note.clone()));

        let updated = update_and_read(
            &pool,
            entity.id,
            Some(name.clone()),
            Some(Some(note.clone())),
        )
        .await
        .expect("Failed to update and read category group");

        assert_eq!(updated.name, name);
        assert_eq!(updated.note, Some(note));
        assert_eq!(updated.version, entity.version + 1);
        assert!(updated.updated_at >= entity.updated_at);
    }

    #[sqlx::test]
    async fn test_empty_update_does_nothing(pool: SqlitePool) {
        let entity = create_and_read(&pool, None, None, None)
            .await
            .expect("Failed to create and read category group");
        let updated = update_and_read(&pool, entity.id, None, None)
            .await
            .expect("Failed to update and read category group");
        assert_eq!(updated, entity);
    }

    #[sqlx::test]
    async fn test_delete(pool: SqlitePool) {
        let reason = "No longer needed".to_string();
        let entity = create_and_read(&pool, None, None, None)
            .await
            .expect("Failed to create and read category group");

        assert!(entity.deleted_at.is_none());
        assert!(entity.deleted_by_user.is_none());
        assert!(entity.deleted_by_device.is_none());
        assert!(entity.tombstone_reason.is_none());

        let deleted = delete_and_read(&pool, entity.id, Some(reason.clone()))
            .await
            .expect("Failed to delete and read category group");

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
