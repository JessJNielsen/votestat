use crate::database::connect_database;
use crate::models::{District, Insert};
use async_trait::async_trait;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct DistrictInput {
    pub name: String,
    pub link: String,
}

#[async_trait]
impl Insert<DistrictInput> for DistrictEntity {
    type Output = DistrictEntity;

    async fn insert(input: &DistrictInput) -> anyhow::Result<Option<DistrictEntity>> {
        let new_district: Option<DistrictEntity> = sqlx::query_as!(
            DistrictEntity,
            r#"
                INSERT OR IGNORE INTO districts (name, link)
                VALUES ($1, $2)
                RETURNING
                district_id as "district_id!",
                name as "name!",
                link as "link!",
                created_at as "created_at!"
            "#,
            input.name,
            input.link,
        )
        .fetch_optional(&connect_database().await)
        .await?;

        println!("new_district {:?}", &new_district);

        Ok(new_district)
    }
}
