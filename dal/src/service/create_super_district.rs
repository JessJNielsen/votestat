use crate::{entities::SuperDistrict, Service};
use chrono::Utc;
use ulid::Ulid;

impl Service {
    pub async fn create_super_district(&self, name: &str) -> anyhow::Result<SuperDistrict> {
        let now = Utc::now();

        let super_district = SuperDistrict {
            super_district_id: Ulid::new().into(),
            name: name.to_string(),
            created_at: now,
        };

        self.repo
            .create_super_district(&self.db, &super_district)
            .await?;

        Ok(super_district)
    }
}
