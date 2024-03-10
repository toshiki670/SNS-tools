use sea_orm_migration::prelude::*;

use super::m20240227_151203_create_item::Item;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Section::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Section::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Section::ItemId).integer().not_null())
                    .col(ColumnDef::new(Section::Name).string())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("section_on_item_id")
                            .from(Section::Table, Section::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Section::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Section {
    Table,
    Id,
    ItemId,
    Name,
}
