use super::Repository;
use crate::database::Queryer;
use crate::entities::SuperDistrict;

impl Repository {
    pub async fn create_super_district<'c, C: Queryer<'c>>(
        &self,
        db: C,
        input: &SuperDistrict,
    ) -> anyhow::Result<()> {
        const QUERY: &str = "INSERT INTO super_districts 
            (super_district_id, name, created_at)
            VALUES ($1, $2, $3)";

        match sqlx::query(QUERY)
            .bind(input.super_district_id)
            .bind(&input.name)
            .bind(input.created_at)
            .execute(db)
            .await
        {
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }
}
