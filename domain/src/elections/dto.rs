use crate::elections::entity;
use crate::elections::entity::ElectionType;
use anyhow::Error;
use chrono::{NaiveDate, Utc};
use sea_orm::ActiveValue;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub struct NewElectionDTO {
    pub r#type: ElectionType,
    pub date: NaiveDate,
}

impl TryFrom<NewElectionDTO> for entity::ActiveModel {
    type Error = Error;

    fn try_from(dto: NewElectionDTO) -> anyhow::Result<Self> {
        Ok(entity::ActiveModel {
            election_id: ActiveValue::Set(Uuid::new_v4()),
            election_type: ActiveValue::Set(dto.r#type),
            election_date: ActiveValue::Set(dto.date),
            created_at: ActiveValue::Set(Utc::now()),
        })
    }
}
