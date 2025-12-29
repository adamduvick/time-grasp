//! Store adapter for the `category` table.
//!
//! This module contains the `sqlx`-backed implementations of the store
//! traits for category-related domain types. The impls translate between
//! domain DTOs and raw SQL queries used by the application.
use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::store::*;
use model::*;

#[async_trait]
impl Creatable<C_Category> for C_Category {
    async fn create(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        sqlx::query(
            "INSERT INTO category (
                id,
                name, 
                note,
                group_id
            ) VALUES (?, ?, ?, ?)",
        )
        .bind(entity.id)
        .bind(&entity.name)
        .bind(&entity.note)
        .bind(entity.group_id)
        .execute(pool)
        .await?;

        Ok(entity.id)
    }
}

#[async_trait]
impl Readable<R_Category> for R_Category {
    type Filter = CategoryFilter;

    const BASE_SELECT: &'static str = r"
        SELECT 
            id,
            name, 
            note, 
            group_id, 
            version, 
            created_at, 
            updated_at,
            deleted_at,
            deleted_by_user,
            deleted_by_device,
            tombstone_reason
        FROM category
    ";

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

impl Filterable for CategoryFilter {
    fn apply(&self, qb: &mut QueryBuilder<Sqlite>) {
        qb.push(" WHERE 1=1");

        let Self { id } = self.clone();

        if let Some(id) = id {
            qb.push(" AND id = ").push_bind(id);
        }
    }
}

#[async_trait]
impl Updatable<U_Category> for U_Category {
    async fn update(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        let Self {
            id,
            name,
            note,
            group_id,
        } = entity;

        let name_flag = name.is_some();
        let note_flag = note.is_some();
        let group_flag = group_id.is_some();

        // No fields requested to change → skip hitting the DB
        if !name_flag && !note_flag && !group_flag {
            return Ok(id);
        }

        let name_value = name;
        let note_value = note.flatten();
        let group_value = group_id;

        sqlx::query(
            "UPDATE category SET
                name = CASE WHEN ? THEN ? ELSE name END,
                note = CASE WHEN ? THEN ? ELSE note END,
                group_id = CASE WHEN ? THEN ? ELSE group_id END
            WHERE id = ?",
        )
        .bind(name_flag)
        .bind(name_value)
        .bind(note_flag)
        .bind(note_value)
        .bind(group_flag)
        .bind(group_value)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(id)
    }
}

#[async_trait]
impl Deletable<D_Category> for D_Category {
    async fn delete(pool: &SqlitePool, entity: Self) -> Result<Uuid> {
        sqlx::query(
            "UPDATE category SET 
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
    use crate::store::create_and_read_group;
    use model::EpochMillis;

    pub async fn create_and_read(
        pool: &SqlitePool,
        id: Option<Uuid>,
        name: Option<String>,
        note: Option<String>,
        group_id: Uuid,
    ) -> Result<R_Category> {
        let entity_id = id.unwrap_or(Uuid::new_v4());
        let entity = C_Category {
            id: entity_id.clone(),
            name: name.unwrap_or("Test R_Category".to_string()),
            note: note.map(|s| s.to_string()),
            group_id,
        };
        C_Category::create(pool, entity).await?;
        R_Category::read(&pool, entity_id).await
    }

    pub async fn update_and_read(
        pool: &SqlitePool,
        id: Uuid,
        name: Option<String>,
        note: Option<Option<String>>,
        group_id: Option<Uuid>,
    ) -> Result<R_Category> {
        let params = U_Category {
            id,
            name,
            note,
            group_id,
        };
        U_Category::update(pool, params).await?;
        R_Category::read(&pool, id).await
    }

    pub async fn delete_and_read(
        pool: &SqlitePool,
        id: Uuid,
        reason: Option<String>,
    ) -> Result<R_Category> {
        let params = D_Category {
            id,
            tombstone_reason: reason.unwrap_or("User request".to_string()),
        };
        D_Category::delete(pool, params).await?;
        R_Category::read(&pool, id).await
    }

    #[sqlx::test]
    async fn test_create(pool: SqlitePool) {
        let group = create_and_read_group(&pool, None, None, None)
            .await
            .expect("Failed to create category group");
        let name = "test category".to_string();
        let note = "test note".to_string();
        let entity = create_and_read(
            &pool,
            None,
            Some(name.clone()),
            Some(note.clone()),
            group.id,
        )
        .await
        .expect("Failed to create and read category");

        assert_eq!(entity.name, name);
        assert_eq!(entity.note, Some(note));
        assert_eq!(entity.group_id, group.id);
        assert_eq!(entity.version, 1);
        assert!((EpochMillis::now() - entity.created_at).0 <= 10); // within 10 milliseconds
        assert_eq!(entity.created_at, entity.updated_at);
        assert!(entity.deleted_at.is_none());
        assert!(entity.deleted_by_user.is_none());
        assert!(entity.deleted_by_device.is_none());
        assert!(entity.tombstone_reason.is_none());
    }

    #[sqlx::test]
    async fn test_create_with_fake_group_fails(pool: SqlitePool) {
        let fake_group_id = Uuid::new_v4();
        let entity = create_and_read(&pool, None, None, None, fake_group_id).await;

        assert!(entity.is_err());
    }

    #[sqlx::test]
    async fn test_create_dupe_id_fails(pool: SqlitePool) {
        let id = Uuid::new_v4();
        let group = create_and_read_group(&pool, None, None, None)
            .await
            .expect("Failed to create category group");
        let _entity1 = create_and_read(
            &pool,
            Some(id),
            Some("R_Category 1".to_string()),
            None,
            group.id,
        )
        .await
        .expect("Failed to create and read category");
        let entity2 = create_and_read(
            &pool,
            Some(id),
            Some("R_Category 2".to_string()),
            None,
            group.id,
        )
        .await;

        assert!(entity2.is_err());
    }

    #[sqlx::test]
    async fn test_create_dupe_name_fails(pool: SqlitePool) {
        let name = "Unique R_Category Name".to_string();
        let group = create_and_read_group(&pool, None, None, None)
            .await
            .expect("Failed to create category group");
        let _entity1 = create_and_read(&pool, None, Some(name.clone()), None, group.id)
            .await
            .expect("Failed to create and read category group");
        let entity2 = create_and_read(&pool, None, Some(name), None, group.id).await;

        assert!(entity2.is_err());
    }

    #[sqlx::test]
    async fn test_update(pool: SqlitePool) {
        let name = "updated name".to_string();
        let note = "updated note".to_string();
        let group1 = create_and_read_group(&pool, None, Some("group1".to_string()), None)
            .await
            .expect("Failed to create group");
        let group2 = create_and_read_group(&pool, None, Some("group2".to_string()), None)
            .await
            .expect("Failed to create group");
        let entity = create_and_read(&pool, None, None, None, group1.id)
            .await
            .expect("Failed to create and read category group");

        assert_ne!(entity.name, name);
        assert_ne!(entity.note, Some(note.clone()));

        let updated = update_and_read(
            &pool,
            entity.id,
            Some(name.clone()),
            Some(Some(note.clone())),
            Some(group2.id),
        )
        .await
        .expect("Failed to update and read category group");

        assert_eq!(updated.name, name);
        assert_eq!(updated.note, Some(note));
        assert_eq!(updated.group_id, group2.id);
        assert_eq!(updated.version, entity.version + 1);
        assert!(updated.updated_at >= entity.updated_at);
    }

    #[sqlx::test]
    async fn test_empty_update_does_nothing(pool: SqlitePool) {
        let group = create_and_read_group(&pool, None, None, None)
            .await
            .expect("Failed to create group");
        let entity = create_and_read(&pool, None, None, None, group.id)
            .await
            .expect("Failed to create and read category");
        let updated = update_and_read(&pool, entity.id, None, None, None)
            .await
            .expect("Failed to update and read category");
        assert_eq!(updated, entity);
    }

    #[sqlx::test]
    async fn test_delete(pool: SqlitePool) {
        let reason = "No longer needed".to_string();
        let group = create_and_read_group(&pool, None, None, None)
            .await
            .expect("Failed to create group");
        let entity = create_and_read(&pool, None, None, None, group.id)
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
