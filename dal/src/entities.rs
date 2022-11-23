use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct SuperDistrict {
    pub super_district_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct District {
    pub district_id: Uuid,
    pub name: String,
    pub link: String,
    pub created_at: DateTime<Utc>,
}
