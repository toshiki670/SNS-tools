use sea_orm_migration::prelude::*;

use super::m20240227_151203_create_item::Item;
use super::m20240227_151347_create_tag::Tag;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ItemTagging::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ItemTagging::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ItemTagging::ItemId).integer().not_null())
                    .col(ColumnDef::new(ItemTagging::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("item_tagging_on_item_id")
                            .from(ItemTagging::Table, ItemTagging::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("item_tagging_on_tag_id")
                            .from(ItemTagging::Table, ItemTagging::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ItemTagging::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ItemTagging {
    Table,
    Id,
    ItemId,
    TagId,
}
