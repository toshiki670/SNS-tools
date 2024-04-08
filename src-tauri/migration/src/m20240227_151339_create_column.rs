use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};

use super::m20240227_151203_create_item::Item;
use super::m20240227_151331_create_section::Section;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Column::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::ItemId).integer())
                    .col(ColumnDef::new(Column::SectionId).integer())
                    .col(ColumnDef::new(Column::Name).string())
                    .col(ColumnDef::new(Column::Value).string())
                    .col(
                        ColumnDef::new(Column::ValueType)
                            .enumeration(Column::ValueType, ValueType::iter()).not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("column_on_item_id")
                            .from(Column::Table, Column::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("column_on_section_id")
                            .from(Column::Table, Column::ItemId)
                            .to(Section::Table, Section::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Column::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Column {
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
