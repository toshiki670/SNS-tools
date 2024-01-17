use crate::utility::types::AnyErrResult;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::{path::PathBuf, str::FromStr};

const DB_FILE_NAME: &'static str = "passwords.db";

pub async fn create_pool(mut app_path: PathBuf) -> AnyErrResult<SqlitePool> {
    app_path.push(DB_FILE_NAME);

    let database_url = app_path
        .to_str()
        .expect("Failed to open database file path.");

    // https://site-builder.wiki/posts/891
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true);

    let sqlite_pool = SqlitePoolOptions::new().connect_with(connection_options).await?;

    Ok(sqlite_pool)
}

/// マイグレーションを行う
pub async fn migrate_database(pool: &SqlitePool) -> AnyErrResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
