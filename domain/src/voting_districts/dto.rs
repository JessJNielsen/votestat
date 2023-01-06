use crate::voting_districts::entity;
use crate::voting_districts::entity::VotingDistrictType;
use anyhow::Error;
use chrono::Utc;
use sea_orm::ActiveValue;
use uuid::Uuid;

pub struct NewVotingDistrictDTO {
    pub r#type: VotingDistrictType,
    pub parent_district_id: Option<Uuid>,
    // pub election_id: Uuid, //TODO
    pub name: String,
}

impl TryFrom<NewVotingDistrictDTO> for entity::ActiveModel {
    type Error = Error;

    fn try_from(dto: NewVotingDistrictDTO) -> anyhow::Result<Self> {
        Ok(entity::ActiveModel {
            district_id: ActiveValue::Set(Uuid::new_v4()),
            parent_district_id: ActiveValue::Set(dto.parent_district_id),
            // election_id: ActiveValue::Set(dto.election_id), // TODO
            district_type: ActiveValue::Set(dto.r#type),
            district_name: ActiveValue::Set(dto.name),
            created_at: ActiveValue::Set(Utc::now()),
        })
    }
}
