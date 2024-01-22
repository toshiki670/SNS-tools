use crate::{
    entities::{
        date_at::DateAt,
        filtered::Filtered,
        id::Id,
        item::{Column, Item, ItemRepositoryInterface, Section, ValueType},
    },
    gateways::models::{ItemRow, WebsiteRow},
};
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use sqlx::SqlitePool;

use super::models::{ColumnRow, SectionRow};

pub struct ItemRepository {
    pool: SqlitePool,
}

#[async_trait]
impl ItemRepositoryInterface for ItemRepository {
    async fn create(&self, new_item: &Item) -> anyhow::Result<Item> {
        todo!()
    }

    async fn find_by_id(&self, id: &crate::entities::id::Id) -> anyhow::Result<Item> {
        let item = find_item(&self.pool, id.0).await?;
        let websites = find_websites(&self.pool, item.id).await?;
        let sections = find_sections(&self.pool, item.id).await?;

        let section_ids = sections.iter().map(|s| s.id).collect();
        let columns = find_columns(&self.pool, section_ids, item.id).await?;

        let website_urls = websites.iter().map(|w| w.url.clone()).collect();
        let sections_for_item = sections
            .iter()
            .map(|s| {
                let cols = columns
                    .iter()
                    .filter(|c| c.section_id == Some(s.id))
                    .map(|c| Column {
                        name: c.name.clone(),
                        value: c.value.clone().map(|s| Filtered(s)),
                        value_type: ValueType::from(c.value_type),
                    })
                    .collect();

                Section {
                    name: s.name.clone(),
                    columns: cols,
                }
            })
            .collect();

        let columns_for_item = columns
            .iter()
            .filter(|c| c.item_id == Some(item.id))
            .map(|c| Column {
                name: c.name.clone(),
                value: c.value.clone().map(|s| Filtered(s)),
                value_type: ValueType::from(c.value_type),
            })
            .collect();

        Ok(Item {
            id: Some(Id(item.id)),
            name: item.name,
            username: item.username,
            password: item.password.map(|s| Filtered(s)),
            websites: website_urls,
            sections: sections_for_item,
            columns: columns_for_item,
            note: item.note,
            created_at: Some(DateAt(Utc.from_utc_datetime(&item.created_at))),
            updated_at: Some(DateAt(Utc.from_utc_datetime(&item.updated_at))),
        })
    }

    async fn update(&self, change_item: &Item) -> anyhow::Result<Item> {
        todo!()
    }
}

async fn find_item(pool: &SqlitePool, id: i64) -> anyhow::Result<ItemRow> {
    let item = sqlx::query_as!(ItemRow, "SELECT * FROM items WHERE id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(item)
}

async fn find_websites(pool: &SqlitePool, item_id: i64) -> anyhow::Result<Vec<WebsiteRow>> {
    let websites: Vec<WebsiteRow> = sqlx::query_as!(
        WebsiteRow,
        "SELECT * FROM websites WHERE item_id = $1",
        item_id
    )
    .fetch_all(pool)
    .await?;
    Ok(websites)
}

async fn find_sections(pool: &SqlitePool, item_id: i64) -> anyhow::Result<Vec<SectionRow>> {
    let websites: Vec<SectionRow> = sqlx::query_as!(
        SectionRow,
        "SELECT * FROM sections WHERE item_id = $1",
        item_id
    )
    .fetch_all(pool)
    .await?;
    Ok(websites)
}

async fn find_columns(
    pool: &SqlitePool,
    section_ids: Vec<i64>,
    item_id: i64,
) -> anyhow::Result<Vec<ColumnRow>> {
    // Summarize section_ids by ","
    let section_ids = section_ids
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let websites: Vec<ColumnRow> = sqlx::query_as!(
        ColumnRow,
        "SELECT * FROM columns WHERE section_id IN ($1) OR item_id = $2",
        section_ids,
        item_id
    )
    .fetch_all(pool)
    .await?;
    Ok(websites)
}
