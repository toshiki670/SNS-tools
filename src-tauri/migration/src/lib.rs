pub use sea_orm_migration::prelude::*;

mod m20240227_151203_create_item;
mod m20240227_151313_create_website;
mod m20240227_151331_create_section;
mod m20240227_151339_create_column;
mod m20240227_151347_create_tag;
mod m20240228_163719_create_item_tagging;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240227_151203_create_item::Migration),
            Box::new(m20240227_151313_create_website::Migration),
            Box::new(m20240227_151331_create_section::Migration),
            Box::new(m20240227_151339_create_column::Migration),
            Box::new(m20240227_151347_create_tag::Migration),
            Box::new(m20240228_163719_create_item_tagging::Migration),
        ]
    }
}
