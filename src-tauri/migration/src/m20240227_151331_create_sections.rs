use sea_orm_migration::prelude::*;

use super::m20240227_151203_create_items::Items;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Sections::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sections::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sections::ItemId).integer().not_null())
                    .col(ColumnDef::new(Sections::Name).string())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("section_on_item_id")
                            .from(Sections::Table, Sections::ItemId)
                            .to(Items::Table, Items::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Sections::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sections {
    Table,
    Id,
    ItemId,
    Name,
}
