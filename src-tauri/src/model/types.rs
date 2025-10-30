use sqlx::FromRow;
use uuid::Uuid;

// v_public_entries
#[derive(Debug, Clone, FromRow)]
pub struct PublicEntry {
    pub global_id: Uuid,
    pub payee: String,
    pub start_time: String,       // or time::OffsetDateTime
    pub end_time: Option<String>, // or Option<time::OffsetDateTime>
    pub memo: Option<String>,
    pub category: String, // category name
}

// v_public_categories
#[derive(Debug, Clone, FromRow)]
pub struct PublicCategory {
    pub global_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    #[sqlx(rename = "group")]
    pub group_name: String, // category group name
}

// v_public_category_groups
#[derive(Debug, Clone, FromRow)]
pub struct PublicCategoryGroup {
    pub global_id: Uuid,
    pub name: String,
    pub note: Option<String>,
}
