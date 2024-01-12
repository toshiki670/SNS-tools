use std::path::PathBuf;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
// pub type ConnectionPooled = PooledConnection<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub const DB_FILE_NAME: &'static str = "passwords.db";

pub fn establish_connection(mut app_path: PathBuf) -> ConnectionPool {
    app_path.push(DB_FILE_NAME);

    let database_url = app_path
        .to_str()
        .expect("Failed to open database file path.");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).unwrap()
}

pub fn run_migration(connection_pool: &ConnectionPool) {
    connection_pool
        .get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .unwrap();
}
