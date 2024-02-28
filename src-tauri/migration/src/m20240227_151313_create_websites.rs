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
                    .table(Websites::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Websites::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Websites::ItemId).integer().not_null())
                    .col(ColumnDef::new(Websites::Url).string().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("website_on_item_id")
                            .from(Websites::Table, Websites::ItemId)
                            .to(Items::Table, Items::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Websites::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Websites {
    Table,
    Id,
    ItemId,
    Url,
}
