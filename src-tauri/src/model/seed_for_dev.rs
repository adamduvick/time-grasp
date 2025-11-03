use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::model::types::{
    Category, CategoryForCreate, CategoryGroup, CategoryGroupForCreate, Entry, EntryForCreate,
};

fn to_ms(dt: DateTime<Utc>) -> i64 {
    dt.timestamp()
}
fn from_ms(ms: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp_millis(ms).expect("invalid millis")
}

pub async fn create_entry(pool: &Pool<Sqlite>, item: EntryForCreate) -> anyhow::Result<()> {
    let start = to_ms(item.start_time);
    let end = item.end_time.map(to_ms);
    let timestamp = to_ms(Utc::now());
    sqlx::query!(
        r#"INSERT INTO entry (uuid, name, start_time, end_time, note, created_at, updated_at, category_uuid)
               SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8
               ON CONFLICT(uuid) DO NOTHING"#,
        item.uuid,
        item.name,
        start,
        end,
        item.note,
        timestamp,
        timestamp,
        item.category_uuid,
    )
    .execute(pool)
    .await
    .context(format!("failed to create entry {:?}", item))?;

    Ok(())
}

pub async fn get_entry(pool: &Pool<Sqlite>, uuid: Uuid) -> anyhow::Result<Option<Entry>> {
    let rec = sqlx::query_as!(
        Entry,
        r#"SELECT 
            uuid, 
            name, 
            start_time      as "start_time: DateTime<Utc>", 
            end_time        as "end_time: Option<DateTime<Utc>>", 
            note,
            category_uuid, 
            created_at      as "created_at: DateTime<Utc>", 
            updated_at      as "updated_at: DateTime<Utc>", 
            deleted_at      as "deleted_at: Option<DateTime<Utc>>", 
            version
           FROM entry
           WHERE uuid = ?1"#,
        uuid
    )
    .fetch_optional(pool)
    .await
    .context(format!("failed to fetch entry {:?}", uuid))?;

    Ok(rec)
}

pub async fn create_category(pool: &Pool<Sqlite>, item: CategoryForCreate) -> anyhow::Result<()> {
    let timestamp = to_ms(Utc::now());
    sqlx::query!(
        r#"INSERT INTO category (uuid, name, note, created_at, updated_at, group_uuid)
               SELECT ?1, ?2, ?3, ?4, ?5, ?6
               ON CONFLICT(uuid) DO NOTHING"#,
        item.uuid,
        item.name,
        item.note,
        timestamp,
        timestamp,
        item.group_uuid,
    )
    .execute(pool)
    .await
    .context(format!("failed to create category {:?}", item))?;

    Ok(())
}

pub async fn get_category(pool: &Pool<Sqlite>, uuid: Uuid) -> anyhow::Result<Option<Category>> {
    let rec = sqlx::query_as!(
        Category,
        r#"SELECT 
            uuid, 
            name, 
            note,
            group_uuid, 
            created_at      as "created_at: DateTime<Utc>", 
            updated_at      as "updated_at: DateTime<Utc>", 
            deleted_at      as "deleted_at: Option<DateTime<Utc>>", 
            version
           FROM category
           WHERE uuid = ?1"#,
        uuid
    )
    .fetch_optional(pool)
    .await
    .context(format!("failed to fetch entry {:?}", uuid))?;

    Ok(rec)
}

pub async fn create_category_group(
    pool: &Pool<Sqlite>,
    item: CategoryGroupForCreate,
) -> anyhow::Result<()> {
    let timestamp = to_ms(Utc::now());
    sqlx::query!(
        r#"INSERT INTO category_group (uuid, name, note, created_at, updated_at)
               SELECT ?1, ?2, ?3, ?4, ?5
               ON CONFLICT(uuid) DO NOTHING"#,
        item.uuid,
        item.name,
        item.note,
        timestamp,
        timestamp,
    )
    .execute(pool)
    .await
    .context(format!("failed to create group {:?}", item))?;

    Ok(())
}

pub async fn get_category_group(
    pool: &Pool<Sqlite>,
    uuid: Uuid,
) -> anyhow::Result<Option<CategoryGroup>> {
    let rec = sqlx::query_as!(
        CategoryGroup,
        r#"SELECT 
            uuid, 
            name, 
            note,
            created_at      as "created_at: DateTime<Utc>", 
            updated_at      as "updated_at: DateTime<Utc>", 
            deleted_at      as "deleted_at: Option<DateTime<Utc>>", 
            version
           FROM category_group
           WHERE uuid = ?1"#,
        uuid
    )
    .fetch_optional(pool)
    .await
    .context(format!("failed to fetch entry {:?}", uuid))?;

    Ok(rec)
}

pub async fn seed_dev_db(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    // --- CATEGORY GROUPS ---
    let personal_group = CategoryGroupForCreate {
        uuid: Uuid::new_v4(),
        name: "Personal".to_string(),
        note: Some("Personal activities and chores".to_string()),
    };
    create_category_group(pool, personal_group.clone()).await?;

    let work_group = CategoryGroupForCreate {
        uuid: Uuid::new_v4(),
        name: "Work".to_string(),
        note: Some("Professional and career-related tasks".to_string()),
    };
    create_category_group(pool, work_group.clone()).await?;

    // --- CATEGORIES ---
    let chore_category = CategoryForCreate {
        uuid: Uuid::new_v4(),
        name: "Chores".to_string(),
        note: Some("Routine life maintenance activities".to_string()),
        group_uuid: Some(personal_group.uuid),
    };
    create_category(pool, chore_category.clone()).await?;

    let music_category = CategoryForCreate {
        uuid: Uuid::new_v4(),
        name: "Music".to_string(),
        note: Some("Guitar, piano, singing, etc".to_string()),
        group_uuid: Some(personal_group.uuid),
    };
    create_category(pool, music_category.clone()).await?;

    let grooming_category = CategoryForCreate {
        uuid: Uuid::new_v4(),
        name: "Grooming".to_string(),
        note: Some("Hygene, style, haircuts, etc".to_string()),
        group_uuid: Some(personal_group.uuid),
    };
    create_category(pool, grooming_category.clone()).await?;

    let workout_category = CategoryForCreate {
        uuid: Uuid::new_v4(),
        name: "💪🏼 Working Out".to_string(),
        note: None,
        group_uuid: Some(personal_group.uuid),
    };
    create_category(pool, workout_category.clone()).await?;

    // --- ENTRY ---
    create_entry(
        pool,
        EntryForCreate {
            uuid: Uuid::new_v4(),
            name: "Morning Workout".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(2),
            end_time: Some(Utc::now() - chrono::Duration::hours(1)),
            note: Some("30 minutes of cardio and strength training".to_string()),
            category_uuid: Some(workout_category.uuid.clone()),
        },
    )
    .await?;

    create_entry(
        pool,
        EntryForCreate {
            uuid: Uuid::new_v4(),
            name: "Guitar Practice".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(5),
            end_time: Some(Utc::now() - chrono::Duration::hours(4)),
            note: Some("Practiced scales and a new song".to_string()),
            category_uuid: Some(grooming_category.uuid.clone()),
        },
    )
    .await?;

    let uuid = Uuid::new_v4();

    create_entry(
        pool,
        EntryForCreate {
            uuid: uuid.clone(),
            name: "House Cleaning".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(8),
            end_time: None,
            note: Some("Vacuumed and dusted the living room".to_string()),
            category_uuid: Some(chore_category.uuid.clone()),
        },
    )
    .await?;

    let entry = get_entry(pool, uuid).await?;
    println!("got entry {:?}", entry);

    let category = get_category(pool, chore_category.uuid.clone()).await?;
    println!("got category {:?}", category);

    let group = get_category_group(pool, personal_group.uuid.clone()).await?;
    println!("got group {:?}", group);

    println!("✅ Seeded dev database with test data.");
    Ok(())
}
