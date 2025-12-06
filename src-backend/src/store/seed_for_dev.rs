use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::store::*;
use model::*;

// region:      --- insert helpers

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

fn dt(dt_str: &str) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%dT%H:%M")
        .unwrap()
        .and_utc()
}

// endregion:   --- insert helpers

// region:      --- example data

fn representative_example() -> Tree {
    let tree = Tree {
        items: vec![
            GroupStruct {
                name: "😋 Wants",
                note: None,
                categories: vec![
                    CategoryStruct {
                        name: "🏐 Sports",
                        note: None,
                        entries: vec![EntryStruct {
                            name: "Volleyball",
                            note: None,
                            start_time: dt("2025-12-05T20:45"),
                            end_time: Some(dt("2025-12-05T22:05")),
                        }],
                    },
                    CategoryStruct {
                        name: "🎱 Rec and Leisure",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🍿 TV and Movies",
                        note: None,
                        entries: vec![],
                    },
                ],
            },
            GroupStruct {
                name: "🛒 Needs",
                note: None,
                categories: vec![
                    CategoryStruct {
                        name: "💼 Work",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🛌 Sleep",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "👨🏼‍🔧 Chores",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🌹 Romance",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🫶🏼 Friends",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🏡 Family",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🧼 Personal Care",
                        note: None,
                        entries: vec![],
                    },
                ],
            },
            GroupStruct {
                name: "🏆 Goals",
                note: None,
                categories: vec![
                    CategoryStruct {
                        name: "🎸 Guitar",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🎹 Piano",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "🎓 Grad School",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "📚 Reading",
                        note: None,
                        entries: vec![],
                    },
                    CategoryStruct {
                        name: "💪🏼 Exercise",
                        note: None,
                        entries: vec![],
                    },
                ],
            },
        ],
    };

    tree
}

// endregion:   --- example data

pub(in crate::store) async fn seed_for_dev(pool: &SqlitePool) -> Result<()> {
    representative_example().build_and_insert(pool).await
}
