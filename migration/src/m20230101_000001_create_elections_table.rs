use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Election::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Election::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Election::Type).string().not_null()) // TODO: Consider using actual enum
                    .col(ColumnDef::new(Election::Date).date().not_null())
                    .col(ColumnDef::new(Election::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Election::Table)
                    .name("idx-elect-type-date")
                    .col(Election::Type)
                    .col(Election::Date)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Election::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Election {
    #[iden = "elections"]
    Table,
    #[iden = "election_id"]
    Id,
    #[iden = "election_type"]
    Type,
    #[iden = "election_date"]
    Date,
    #[iden = "created_at"]
    CreatedAt,
}
