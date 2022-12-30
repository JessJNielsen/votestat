//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "election_type")]
pub enum ElectionType {
    #[sea_orm(string_value = "Parliament")]
    Parliament,
    #[sea_orm(string_value = "Municipal")]
    Municipal,
    #[sea_orm(string_value = "Regional")]
    Regional,
    #[sea_orm(string_value = "European")]
    European,
    // Referendum
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "elections")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub election_id: Uuid,
    pub election_type: ElectionType,
    pub election_date: Date,
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
