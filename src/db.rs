use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;

use crate::cmd::models::FileInfo;

pub fn init_db() {
    if PathBuf::from("./.db").exists() {
        fs::remove_file("./pt.db").unwrap();
    }
    let conn:Connection = get_conn();
    let init_sql = include_str!("../sql/init.sql");
    conn.execute_batch(init_sql).expect("init error");
}

pub fn get_conn() -> Connection {
    Connection::open("./pt.db").unwrap()
}

pub fn find_same_name() -> Vec<FileInfo> {
    let sql = "select source.name,source.path,source.ext from source_data source \
    ,target_data target where source.name = target.name";
    let conn = get_conn();
   let mut stat = conn.prepare(sql).unwrap();
   let rows =  stat.query_map([],|row|{
        Ok(FileInfo{
            name: row.get_unwrap(0),
            path: row.get_unwrap(1),
            ext: row.get_unwrap(2),
        })
    }).unwrap();
    let mut file_infos = Vec::new();
    for result in rows {
        file_infos.push(result.unwrap());
    }
    file_infos
}