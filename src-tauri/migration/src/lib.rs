pub use sea_orm_migration::prelude::*;

mod m20240227_151203_create_items;
mod m20240227_151313_create_websites;
mod m20240227_151331_create_sections;
mod m20240227_151339_create_columns;
mod m20240227_151347_create_tags;
mod m20240228_163719_create_item_taggings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240227_151203_create_items::Migration),
            Box::new(m20240227_151313_create_websites::Migration),
            Box::new(m20240227_151331_create_sections::Migration),
            Box::new(m20240227_151339_create_columns::Migration),
            Box::new(m20240227_151347_create_tags::Migration),
            Box::new(m20240228_163719_create_item_taggings::Migration),
        ]
    }
}
