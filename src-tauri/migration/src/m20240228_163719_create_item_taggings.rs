use sea_orm_migration::prelude::*;

use super::m20240227_151203_create_items::Items;
use super::m20240227_151347_create_tags::Tags;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ItemTaggings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ItemTaggings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ItemTaggings::ItemId).integer().not_null())
                    .col(ColumnDef::new(ItemTaggings::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("item_tagging_on_item_id")
                            .from(ItemTaggings::Table, ItemTaggings::ItemId)
                            .to(Items::Table, Items::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("item_tagging_on_tag_id")
                            .from(ItemTaggings::Table, ItemTaggings::TagId)
                            .to(Tags::Table, Tags::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ItemTaggings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ItemTaggings {
    Table,
    Id,
    ItemId,
    TagId,
}
