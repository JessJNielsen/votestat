extern crate database;

use crate::voting_districts::{entity, NewVotingDistrictDTO, VotingDistrict};
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::convert::TryFrom;
use uuid::Uuid;

pub struct VotingDistrictService {
    db: database::DatabaseConnection,
}

impl VotingDistrictService {
    pub fn new(db: database::DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_voting_district(
        &self,
        voting_district: NewVotingDistrictDTO,
    ) -> anyhow::Result<entity::Model> {
        let voting_district = entity::ActiveModel::try_from(voting_district)?;
        let voting_district = voting_district.insert(&self.db).await?;
        // TODO: add parent_district_id for superVotingDistrict

        Ok(voting_district)
    }

    pub async fn find(&self) -> anyhow::Result<Vec<entity::Model>> {
        let voting_districts: Vec<entity::Model> = VotingDistrict::find().all(&self.db).await?;

        Ok(voting_districts)
    }

    pub async fn find_one(&self, id: &Uuid) -> anyhow::Result<Option<entity::Model>> {
        let voting_district: Option<entity::Model> =
            VotingDistrict::find_by_id(*id).one(&self.db).await?;

        Ok(voting_district)
    }

    // pub async fn update(&self, id: &Uuid, user: dto::NewVotingDistrictDTO) -> Result<VotingDistrict> {
    //     let user = VotingDistrict::try_from(user)?;
    //
    //     self.user_repository.update(id, &user).await
    // }
}
