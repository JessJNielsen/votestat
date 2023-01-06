extern crate database;

use crate::elections::{entity, Election, NewElectionDTO};
use sea_orm::{query::*, sea_query, ColumnTrait, DbErr, EntityTrait};
use std::convert::TryFrom;
use uuid::Uuid;

pub struct ElectionService {
    db: database::DatabaseConnection,
}

impl ElectionService {
    pub fn new(db: database::DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_election(
        &self,
        election_dto: NewElectionDTO,
    ) -> anyhow::Result<entity::Model> {
        let election = entity::ActiveModel::try_from(election_dto)?;
        let insert_result = entity::Entity::insert(election.clone())
            .on_conflict(
                sea_query::OnConflict::columns([
                    entity::Column::ElectionType,
                    entity::Column::ElectionDate,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await;

        match insert_result {
            Ok(model) => Ok(model),
            Err(DbErr::RecordNotFound(error)) => {
                println!("{}", error);
                let existing = entity::Entity::find()
                    .filter(entity::Column::ElectionType.eq(election_dto.r#type))
                    .filter(entity::Column::ElectionDate.eq(election_dto.date))
                    .one(&self.db)
                    .await?;

                match existing {
                    Some(model) => Ok(model),
                    None => panic!("Existing election not found"),
                }
            }
            _ => panic!("Unexpected Error kind"), // TODO improve
        }
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
