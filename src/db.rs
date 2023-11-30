use std::error::Error;
use std::fs;
use std::path::PathBuf;

use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::cmd::models::{FileInfo, TargetData};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub fn init_db() -> SqliteConnection {
    if PathBuf::from("./pt.db").exists() {
        fs::remove_file("./pt.db").unwrap();
    }
    let mut conn:SqliteConnection = SqliteConnection::establish("./pt.db").unwrap();
    run_migrations(&mut conn).expect("init error");
    conn
}

fn run_migrations(conn:&mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn get_conn() -> SqliteConnection {
    SqliteConnection::establish("./pt.db").unwrap()
}

pub fn find_same_name() -> Vec<FileInfo> {
    let sql = "select source.name,source.path,source.ext from source_data source \
    ,target_data target where source.name = target.name";
    let mut conn = get_conn();
    let file_infos:Vec<FileInfo> = diesel::sql_query(sql).load(&mut conn).expect("selete error");
    file_infos
}