use chrono::NaiveDateTime;

pub(super) struct ItemRow {
    pub(super) id: i64,
    pub(super) name: String,
    pub(super) username: Option<String>,
    pub(super) password: Option<String>,
    pub(super) note: Option<String>,

    /// # Example:
    /// `2018-12-07T19:31:28`
    pub(super) created_at: NaiveDateTime,

    /// # Example:
    /// `2018-12-07T19:31:28`
    pub(super) updated_at: NaiveDateTime,
}

pub(super) struct WebsiteRow {
    pub(super) id: i64,
    pub(super) item_id: i64,
    pub(super) url: String,
}

pub(super) struct SectionRow {
    pub(super) id: i64,
    pub(super) item_id: i64,
    pub(super) name: Option<String>,
}

pub(super) struct ColumnRow {
    pub(super) id: i64,
    pub(super) item_id: Option<i64>,
    pub(super) section_id: Option<i64>,
    pub(super) name: Option<String>,
    pub(super) value: Option<String>,
    pub(super) value_type: i64,
}
