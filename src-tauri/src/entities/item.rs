use crate::entities::{date_at::DateAt, id::Id, filtered::Filtered};
use serde::{Deserialize, Serialize};

pub trait ItemRepositoryInterface<E> {
    fn create(&self, new_item: &Item) -> anyhow::Result<Item>;
    fn find_by_id(&self, id: &Id) -> anyhow::Result<Item>;
    fn update(&self, change_item: &Item) -> anyhow::Result<Item>;
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub id: Option<Id>,
    pub name: String,
    pub username: String,
    pub password: Filtered<String>,
    pub website: Vec<String>,
    pub sections: Vec<Sections>,
    pub columns: Vec<Column>,
    pub note: String,
    pub created_at: Option<DateAt>,
    pub updated_at: Option<DateAt>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sections {
    name: String,
    columns: Vec<Column>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Column {
    name: String,
    value: Filtered<String>,
    value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ValueType {
    Undefined = 0,
    Text = 1,
    Url = 2,
    Email = 3,
    Address = 4,
    Date = 5,
    Password = 6,
}

impl Item {
    pub fn create(&mut self) {
        self.created_at = Some(DateAt::now());
        self.updated_at = Some(DateAt::now());
    }

    pub fn update(&mut self) {
        self.updated_at = Some(DateAt::now());
    }
}
