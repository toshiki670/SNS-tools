use chrono::NaiveDateTime;

pub struct ItemRow {
    pub id: i64,
    pub name: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub note: Option<String>,

    /// # Example:
    /// `2018-12-07T19:31:28`
    pub created_at: NaiveDateTime,

    /// # Example:
    /// `2018-12-07T19:31:28`
    pub updated_at: NaiveDateTime,
}

pub struct WebsiteRow {
    pub id: i64,
    pub item_id: i64,
    pub url: String,
}

pub struct SectionRow {
    pub id: i64,
    pub item_id: i64,
    pub name: Option<String>,
}

pub struct ColumnRow {
    pub id: i64,
    pub item_id: Option<i64>,
    pub section_id: Option<i64>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub value_type: i64,
}
