use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};

use super::m20240227_151203_create_items::Items;
use super::m20240227_151331_create_sections::Sections;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Columns::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Columns::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Columns::ItemId).integer())
                    .col(ColumnDef::new(Columns::SectionId).integer())
                    .col(ColumnDef::new(Columns::Name).string())
                    .col(ColumnDef::new(Columns::Value).string())
                    .col(
                        ColumnDef::new(Columns::ValueType)
                            .enumeration(Columns::ValueType, ValueType::iter()).not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("column_on_item_id")
                            .from(Columns::Table, Columns::ItemId)
                            .to(Items::Table, Items::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("column_on_section_id")
                            .from(Columns::Table, Columns::ItemId)
                            .to(Sections::Table, Sections::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Columns::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Columns {
    Table,
    Id,
    ItemId,
    SectionId,
    Name,
    Value,
    ValueType,
}

#[derive(Iden, EnumIter)]
enum ValueType {
    Undefined = 0,
    Text = 1,
    Url = 2,
    Email = 3,
    Address = 4,
    Date = 5,
    Password = 6,
}
