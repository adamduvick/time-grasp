use chrono::DateTime;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::store::*;
use model::model::*;

struct Tree {
    items: Vec<GroupStruct>,
}
struct GroupStruct {
    name: &'static str,
    note: Option<&'static str>,
    categories: Vec<CategoryStruct>,
}
struct CategoryStruct {
    name: &'static str,
    note: Option<&'static str>,
    entries: Vec<EntryStruct>,
}
struct EntryStruct {
    name: &'static str,
    note: Option<&'static str>,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl Tree {
    async fn build_and_insert(self, pool: &SqlitePool) -> Result<()> {
        for group_params in self.items.into_iter() {
            let GroupStruct {
                name,
                note,
                categories,
            } = group_params;

            let group_id = C_Group::create(
                pool,
                C_Group {
                    id: Uuid::new_v4(),
                    name: name.into(),
                    note: note.map(|note| note.into()),
                },
            )
            .await?;

            for category_params in categories.into_iter() {
                let CategoryStruct {
                    name,
                    note,
                    entries,
                } = category_params;
                let category_id = C_Category::create(
                    pool,
                    C_Category {
                        id: Uuid::new_v4(),
                        name: name.into(),
                        note: note.map(|note| note.into()),
                        group_id,
                    },
                )
                .await?;

                for entry_params in entries.into_iter() {
                    let EntryStruct {
                        name,
                        note,
                        start_time,
                        end_time,
                    } = entry_params;
                    let id = C_Entry::create(
                        pool,
                        C_Entry {
                            id: Uuid::new_v4(),
                            name: name.into(),
                            note: note.map(|note| note.into()),
                            start_time: start_time.into(),
                            end_time: end_time.map(|end_time| end_time.into()),
                            category_id,
                        },
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }
}

pub(in crate::store) async fn seed_for_dev(pool: &SqlitePool) -> Result<()> {
    let group_names = vec!["😋 Wants", "🛒 Needs", "🏆 Goals"];
    let wants_category_names = vec!["🏐 Sports", "🎱 Rec and Leisure", "🍿 TV and Movies"];
    let needs_category_names = vec![
        "💼 Work",
        "🛌 Sleep",
        "👨🏼‍🔧 Chores",
        "🌹 Romance",
        "🫶🏼 Friends",
        "🏡 Family",
        "🧼 Personal Care",
    ];
    let goals_category_names = vec![
        "🎸 Guitar",
        "🎹 Piano",
        "🎓 Grad School",
        "📚 Reading",
        "💪🏼 Exercise",
    ];

    let group_id = C_Group::create(
        &pool,
        C_Group {
            id: Uuid::new_v4(),
            name: group_names.get(0).unwrap().to_string(),
            note: None,
        },
    );

    Ok(())
}
