use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::Username).string())
                    .col(ColumnDef::new(Item::Password).string())
                    .col(ColumnDef::new(Item::Note).string())
                    .col(ColumnDef::new(Item::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Item::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Item {
    Table,
    Id,
    Name,
    Username,
    Password,
    Note,
    CreatedAt,
    UpdatedAt,
}
