use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "voting_district_type"
)]
pub enum VotingDistrictType {
    #[sea_orm(string_value = "Super")]
    Super,
    #[sea_orm(string_value = "Local")]
    Local,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "voting_districts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub district_id: Uuid,
    #[sea_orm(indexed, nullable)]
    pub parent_district_id: Option<Uuid>,
    // pub election_id: Uuid, // TODO
    pub district_type: VotingDistrictType,
    pub district_name: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(has_many = "super::super_districts::Entity")]
    // SuperDistricts,
}

// impl Related<super::super_districts::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::SuperDistricts.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
