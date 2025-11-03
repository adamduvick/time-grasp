use anyhow::Context;
use chrono::{DateTime, Utc};
// src-tauri/src/model/seed_for_dev.rs
use sqlx::types::Uuid;
use sqlx::{Pool, Sqlite};

use crate::model::types::{CategoryForCreate, CategoryGroupForCreate, EntryForCreate};

fn to_ms(dt: DateTime<Utc>) -> i64 {
    dt.timestamp_millis()
}
fn from_ms(ms: i64) -> DateTime<Utc> {
    chrono::DateTime::<Utc>::from_timestamp_millis(ms).expect("invalid millis")
}

pub async fn create_entry(pool: &Pool<Sqlite>, item: EntryForCreate) -> anyhow::Result<()> {
    let start = to_ms(item.start_time);
    let end = item.end_time.map(to_ms);
    let timestamp = to_ms(Utc::now());
    sqlx::query!(
        r#"INSERT INTO entry (global_id, name, start_time, end_time, note, created_at, updated_at, category_id)
               SELECT ?1, ?2, ?3, ?4, ?5, ?6, ?7, c.id FROM category c WHERE c.global_id = ?8
               ON CONFLICT(global_id) DO NOTHING"#,
        item.global_id,
        item.name,
        start,
        end,
        item.note,
        timestamp,
        timestamp,
        item.category_global_id,
    )
    .execute(pool)
    .await
    .context(format!("failed to create entry {:?}", item))?;

    Ok(())
}

pub async fn create_category(pool: &Pool<Sqlite>, item: CategoryForCreate) -> anyhow::Result<()> {
    let timestamp = to_ms(Utc::now());
    sqlx::query!(
        r#"INSERT INTO category (global_id, name, note, created_at, updated_at, group_id)
               SELECT ?1, ?2, ?3, ?4, ?5, g.id FROM category_group g WHERE g.global_id = ?6
               ON CONFLICT(global_id) DO NOTHING"#,
        item.global_id,
        item.name,
        item.note,
        timestamp,
        timestamp,
        item.group_global_id,
    )
    .execute(pool)
    .await
    .context(format!("failed to create category {:?}", item))?;

    Ok(())
}

pub async fn create_category_group(
    pool: &Pool<Sqlite>,
    item: CategoryGroupForCreate,
) -> anyhow::Result<()> {
    let timestamp = to_ms(Utc::now());
    sqlx::query!(
        r#"INSERT INTO category_group (global_id, name, note, created_at, updated_at)
               SELECT ?1, ?2, ?3, ?4, ?5
               ON CONFLICT(global_id) DO NOTHING"#,
        item.global_id,
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

pub async fn seed_dev_db(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    // --- CATEGORY GROUPS ---
    let personal_group = CategoryGroupForCreate {
        global_id: Uuid::new_v4(),
        name: "Personal".to_string(),
        note: Some("Personal activities and chores".to_string()),
    };
    create_category_group(pool, personal_group.clone()).await?;

    let work_group = CategoryGroupForCreate {
        global_id: Uuid::new_v4(),
        name: "Work".to_string(),
        note: Some("Professional and career-related tasks".to_string()),
    };
    create_category_group(pool, work_group.clone()).await?;

    // --- CATEGORIES ---
    let chore_category = CategoryForCreate {
        global_id: Uuid::new_v4(),
        name: "Chores".to_string(),
        note: Some("Routine life maintenance activities".to_string()),
        group_global_id: Some(personal_group.global_id),
    };
    create_category(pool, chore_category.clone()).await?;

    let music_category = CategoryForCreate {
        global_id: Uuid::new_v4(),
        name: "Music".to_string(),
        note: Some("Guitar, piano, singing, etc".to_string()),
        group_global_id: Some(personal_group.global_id),
    };
    create_category(pool, music_category.clone()).await?;

    let grooming_category = CategoryForCreate {
        global_id: Uuid::new_v4(),
        name: "Grooming".to_string(),
        note: Some("Hygene, style, haircuts, etc".to_string()),
        group_global_id: Some(personal_group.global_id),
    };
    create_category(pool, grooming_category.clone()).await?;

    let workout_category = CategoryForCreate {
        global_id: Uuid::new_v4(),
        name: "💪🏼 Working Out".to_string(),
        note: None,
        group_global_id: Some(personal_group.global_id),
    };
    create_category(pool, workout_category.clone()).await?;

    // --- CATEGORIES ---
    create_entry(
        pool,
        EntryForCreate {
            global_id: Uuid::new_v4(),
            name: "Morning Workout".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(2),
            end_time: Some(Utc::now() - chrono::Duration::hours(1)),
            note: Some("30 minutes of cardio and strength training".to_string()),
            category_global_id: Some(workout_category.global_id.clone()),
        },
    )
    .await?;
    create_entry(
        pool,
        EntryForCreate {
            global_id: Uuid::new_v4(),
            name: "Guitar Practice".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(5),
            end_time: Some(Utc::now() - chrono::Duration::hours(4)),
            note: Some("Practiced scales and a new song".to_string()),
            category_global_id: Some(workout_category.global_id.clone()),
        },
    )
    .await?;
    create_entry(
        pool,
        EntryForCreate {
            global_id: Uuid::new_v4(),
            name: "House Cleaning".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(8),
            end_time: None,
            note: Some("Vacuumed and dusted the living room".to_string()),
            category_global_id: Some(chore_category.global_id.clone()),
        },
    )
    .await?;

    println!("✅ Seeded dev database with test data.");
    Ok(())
}
