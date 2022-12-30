extern crate database;

use crate::elections::{entity, Election, NewElectionDTO};
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::convert::TryFrom;
use uuid::Uuid;

pub struct ElectionService {
    db: database::DatabaseConnection,
}

impl ElectionService {
    pub fn new(db: database::DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_election(&self, election: NewElectionDTO) -> anyhow::Result<entity::Model> {
        let election = entity::ActiveModel::try_from(election)?;
        let election = election.insert(&self.db).await?;

        Ok(election)
    }

    pub async fn find(&self) -> anyhow::Result<Vec<entity::Model>> {
        let elections: Vec<entity::Model> = Election::find().all(&self.db).await?;

        Ok(elections)
    }

    pub async fn find_one(&self, id: &Uuid) -> anyhow::Result<Option<entity::Model>> {
        let election: Option<entity::Model> = Election::find_by_id(*id).one(&self.db).await?;

        Ok(election)
    }

    // pub async fn update(&self, id: &Uuid, user: dto::NewElectionDTO) -> Result<Election> {
    //     let user = Election::try_from(user)?;
    //
    //     self.user_repository.update(id, &user).await
    // }
}
