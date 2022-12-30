use crate::elections::entity;
use crate::elections::entity::ElectionType;
use anyhow::Error;
use chrono::{DateTime, NaiveDate, Utc};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use uuid::Uuid;

pub struct NewElectionDTO {
    pub r#type: ElectionType,
    pub date: NaiveDate,
}

impl TryFrom<NewElectionDTO> for entity::ActiveModel {
    type Error = Error;

    fn try_from(dto: NewElectionDTO) -> anyhow::Result<Self> {
        Ok(entity::ActiveModel {
            election_id: Set(Uuid::new_v4()),
            election_type: Set(dto.r#type),
            election_date: Set(dto.date),
            created_at: Set(Utc::now()),
        })
    }
}
