use ::entity::prelude::{Column, Item, Section, Tag, Website};
use ::entity::{column, item, section, tag, website};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<
        Option<(
            item::Model,
            Vec<website::Model>,
            Vec<(section::Model, Vec<column::Model>)>,
            Vec<column::Model>,
            Vec<tag::Model>,
        )>,
        DbErr,
    > {
        let item: Option<item::Model> = Item::find_by_id(id).one(db).await?;
        let item = if let Some(i) = item {
            i
        } else {
            return Ok(None);
        };

        let websites = item.find_related(Website).all(db).await?;
        let sections = item.find_related(Section).all(db).await?;
        let section_ids: Vec<i32> = sections.iter().map(|s| s.id).collect();

        let columns = Column::find()
            .filter(
                Condition::any()
                    .add(column::Column::ItemId.eq(item.id))
                    .add(column::Column::SectionId.is_in(section_ids)),
            )
            .all(db)
            .await?;

        let section_with_columns = sections
            .iter()
            .map(|section| {
                let columns: Vec<column::Model> = columns
                    .iter()
                    .cloned()
                    .filter(|c| c.section_id.is_some())
                    .filter(|c| c.section_id.unwrap() == section.id)
                    .collect();
                (section.clone(), columns)
            })
            .collect();

        let item_columns: Vec<column::Model> = columns
            .iter()
            .cloned()
            .filter(|c| c.item_id.is_some())
            .filter(|c| c.item_id.unwrap() == item.id)
            .collect();

        let tags = item.find_related(Tag).all(db).await?;

        Ok(Some((item, websites, section_with_columns, item_columns, tags)))
    }
}
