use crate::entities::{date_at::DateAt, filtered::Filtered, id::Id};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ItemRepositoryInterface {
    async fn create(&self, new_item: &Item) -> anyhow::Result<Item>;
    async fn find_by_id(&self, id: &Id) -> anyhow::Result<Item>;
    async fn update(&self, change_item: &Item) -> anyhow::Result<Item>;
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub id: Option<Id>,
    pub name: String,
    pub username: Option<String>,
    pub password: Option<Filtered<String>>,
    pub websites: Vec<String>,
    pub sections: Vec<Section>,
    pub columns: Vec<Column>,
    pub note: Option<String>,
    pub created_at: Option<DateAt>,
    pub updated_at: Option<DateAt>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Section {
    pub name: Option<String>,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Column {
    pub name: Option<String>,
    pub value: Option<Filtered<String>>,
    pub value_type: ValueType,
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

impl From<i64> for ValueType {
    fn from(value: i64) -> Self {
        match value {
            1 => ValueType::Text,
            2 => ValueType::Url,
            3 => ValueType::Email,
            4 => ValueType::Address,
            5 => ValueType::Date,
            6 => ValueType::Password,
            _ => ValueType::Undefined, // 未知の値に対するデフォルトの処理
        }
    }
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
