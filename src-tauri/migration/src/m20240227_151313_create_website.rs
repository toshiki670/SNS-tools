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
                    .table(Website::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Website::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Website::ItemId).integer().not_null())
                    .col(ColumnDef::new(Website::Url).string().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("website_on_item_id")
                            .from(Website::Table, Website::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Website::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Website {
    Table,
    Id,
    ItemId,
    Url,
}
