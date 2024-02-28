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
                    .table(Items::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Items::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Items::Name).string().not_null())
                    .col(ColumnDef::new(Items::Username).string())
                    .col(ColumnDef::new(Items::Password).string())
                    .col(ColumnDef::new(Items::Note).string())
                    .col(ColumnDef::new(Items::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Items::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Items {
    Table,
    Id,
    Name,
    Username,
    Password,
    Note,
    CreatedAt,
    UpdatedAt,
}
