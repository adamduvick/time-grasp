use chrono::DateTime;
use chrono::Months;
use chrono::NaiveDateTime;
use chrono::TimeDelta;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::store::*;
use model::*;

// region:      --- insert helpers

struct Tree<S: Into<String>> {
    items: Vec<GroupStruct<S>>,
}
struct GroupStruct<S: Into<String>> {
    name: S,
    note: Option<S>,
    categories: Vec<CategoryStruct<S>>,
}
struct CategoryStruct<S: Into<String>> {
    name: S,
    note: Option<S>,
    entries: Vec<EntryStruct<S>>,
}
struct EntryStruct<S: Into<String>> {
    name: S,
    note: Option<S>,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl<S: Into<String>> Tree<S> {
    async fn build_and_insert(self, pool: &SqlitePool) -> Result<()> {
        for group_fields in self.items.into_iter() {
            let group = C_Group {
                id: Uuid::new_v4(),
                name: group_fields.name.into(),
                note: group_fields.note.map(|note| note.into()),
            };
            let group_id = C_Group::create(pool, group).await?;

            for category_fields in group_fields.categories.into_iter() {
                let category = C_Category {
                    id: Uuid::new_v4(),
                    name: category_fields.name.into(),
                    note: category_fields.note.map(|note| note.into()),
                    group_id,
                };
                let category_id = C_Category::create(pool, category).await?;

                for entry_fields in category_fields.entries.into_iter() {
                    let entry = C_Entry {
                        id: Uuid::new_v4(),
                        name: entry_fields.name.into(),
                        note: entry_fields.note.map(|note| note.into()),
                        start_time: entry_fields.start_time.into(),
                        end_time: entry_fields.end_time.map(|end_time| end_time.into()),
                        category_id,
                    };
                    let _entry_id = C_Entry::create(pool, entry).await?;
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

fn representative_example<S: Into<String>>() -> Tree<&'static str> {
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

use rand::prelude::IndexedMutRandom;

fn random_item_mut<T>(v: &mut Vec<T>) -> &mut T {
    let mut rng = rand::rng();
    v.choose_mut(&mut rng).unwrap()
}

fn random_data<S: Into<String>>() -> Tree<String> {
    let mut tree = Tree { items: vec![] };

    let (n_groups, n_categories, n_entries) = (5, 25, 1000);
    let mut durations = vec![15, 30, 60, 120, 240, 480];
    let mut time_counter = Utc::now().checked_sub_months(Months::new(3)).unwrap();

    for i in 0..n_groups {
        tree.items.push(GroupStruct {
            name: format!("Group {:04}", i + 1),
            note: None,
            categories: vec![],
        });
    }
    for i in 0..n_categories {
        let group = random_item_mut(&mut tree.items);
        group.categories.push(CategoryStruct {
            name: format!("Category {:04}", i + 1),
            note: None,
            entries: vec![],
        })
    }
    for i in 0..n_entries {
        let group = random_item_mut(&mut tree.items);
        let category = random_item_mut(&mut group.categories);
        let duration = random_item_mut(&mut durations).clone() * 60;
        let start_time = time_counter.clone();
        let end_time = start_time
            .checked_add_signed(TimeDelta::new(duration, 0).unwrap())
            .unwrap();
        time_counter = end_time.clone();
        category.entries.push(EntryStruct {
            name: format!("Entry {:04}", i + 1),
            note: None,
            start_time,
            end_time: Some(end_time),
        })
    }

    tree
}

// endregion:   --- example data

pub(in crate::store) async fn seed_for_dev(pool: &SqlitePool) -> Result<()> {
    // representative_example::<&'static str>()
    //     .build_and_insert(pool)
    //     .await
    random_data::<String>().build_and_insert(pool).await
}
