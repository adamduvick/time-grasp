use anyhow::Context;
// src-tauri/src/model/seed_for_dev.rs
use rand::seq::IndexedRandom;
use rand::Rng;
use sqlx::types::Uuid;
use sqlx::{Pool, Sqlite};
use time::{Duration, OffsetDateTime};

pub async fn seed_dev_db(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    // --- CATEGORY GROUPS ---
    let group_ids: Vec<(i64, Uuid, &str, &str)> = vec![
        (
            1,
            Uuid::new_v4(),
            "Work",
            "Professional and career-related tasks",
        ),
        (
            2,
            Uuid::new_v4(),
            "Personal",
            "Personal activities and chores",
        ),
    ];

    for (id, global_id, name, note) in &group_ids {
        let gid = global_id.to_string(); // keep it alive across .await

        sqlx::query!(
            r#"INSERT INTO category_groups (id, global_id, name, note)
               VALUES (?1, ?2, ?3, ?4)
               ON CONFLICT(global_id) DO NOTHING"#,
            id,
            gid,
            name,
            note
        )
        .execute(pool)
        .await
        .context("group seeding failed")?;
    }

    // --- CATEGORIES ---
    let category_defs: Vec<(i64, Uuid, &str, &str, i64)> = vec![
        (1, Uuid::new_v4(), "Coding", "Software development tasks", 1),
        (
            2,
            Uuid::new_v4(),
            "Meetings",
            "Internal or external meetings",
            1,
        ),
        (
            3,
            Uuid::new_v4(),
            "Exercise",
            "Health-related physical activity",
            2,
        ),
        (
            4,
            Uuid::new_v4(),
            "Chores",
            "Household or daily maintenance tasks",
            2,
        ),
    ];

    for (id, global_id, name, note, group_id) in &category_defs {
        let gid = global_id.to_string(); // keep it alive across .await

        sqlx::query!(
            r#"INSERT INTO categories (id, global_id, name, note, group_id)
               VALUES (?1, ?2, ?3, ?4, ?5)
               ON CONFLICT(global_id) DO NOTHING"#,
            id,
            gid,
            name,
            note,
            group_id
        )
        .execute(pool)
        .await
        .context("category seeding failed")?;
    }

    // --- ENTRIES ---
    let payees = [
        "Work Project A",
        "Work Project B",
        "Cleaning",
        "Groceries",
        "Gym",
        "Running",
    ];
    let memos = [
        Some("Morning session"),
        Some("Evening wrap-up"),
        Some("Quick workout"),
        Some("Weekly sync"),
        None,
    ];

    let mut rng = rand::rng();
    let now = OffsetDateTime::now_utc();

    for i in 0..10 {
        let start_offset = Duration::hours(rng.random_range(1..100));
        let start = now - start_offset;
        let end = start + Duration::minutes(rng.random_range(30..180));

        let gid = Uuid::new_v4().to_string(); // keep it alive across .await
        let payee = payees.choose(&mut rng).unwrap();
        let start_time = start.format(&time::format_description::well_known::Rfc3339)?;
        let end_time = end.format(&time::format_description::well_known::Rfc3339)?;
        let memo = memos.choose(&mut rng).unwrap_or(&None).clone();
        let category_id = rng.random_range(1..=4);

        sqlx::query!(
            r#"INSERT INTO entries (global_id, payee, start_time, end_time, memo, category_id)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
            gid,
            payee,
            start_time,
            end_time,
            memo,
            category_id
        )
        .execute(pool)
        .await
        .context("entry seeding failed")?;
    }

    println!("✅ Seeded dev database with test data.");
    Ok(())
}
