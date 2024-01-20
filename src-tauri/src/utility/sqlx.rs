// Cipher: https://www.zetetic.net/sqlcipher/sqlcipher-api/
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::{path::PathBuf, str::FromStr};

const DB_FILE_NAME: &'static str = "passwords.db";

pub async fn create_pool(mut app_path: PathBuf) -> anyhow::Result<SqlitePool> {
    app_path.push(DB_FILE_NAME);

    let database_url = app_path
        .to_str()
        .expect("Failed to open database file path.");

    let connection_options = SqliteConnectOptions::from_str(database_url)?
        .pragma("key", "the_password")
        .create_if_missing(true) //DB_FILE_NAME がなかった場合に作成
        .journal_mode(SqliteJournalMode::Wal) // See: https://site-builder.wiki/posts/891
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true);

    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}

pub async fn migrate_database(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
