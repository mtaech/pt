use std::fmt::format;
use std::fs;
use std::path::PathBuf;

use crate::cmd::models::FileInfo;
use rusqlite::{params, Connection};

pub fn init_db() {
    if PathBuf::from("./.db").exists() {
        fs::remove_file("./pt.db").unwrap();
    }
    let conn: Connection = get_conn();
    let init_sql = include_str!("../../sql/init.sql");
    conn.execute_batch(init_sql).expect("init error");
}

pub fn get_conn() -> Connection {
    Connection::open("./pt.db").unwrap()
}

///查找同名文件
pub fn find_same_name(source_type: &str, target_type: &str) -> Vec<FileInfo> {
    let sql = format!(
        r#"select source.name,source.path,source.ext
            from source_data source
            where source.name in (select compare.name from compare_data compare
             where compare.ext = lower('{}'))
            and source.ext = lower('{}')"#,
        target_type, source_type
    );
    println!("same sql {}", &sql);
    let conn = get_conn();
    let mut stat = conn.prepare(&sql).unwrap();
    let rows = stat
        .query_map([], |row| {
            Ok(FileInfo {
                name: row.get_unwrap(0),
                path: row.get_unwrap(1),
                ext: row.get_unwrap(2),
            })
        })
        .unwrap();
    let mut file_infos = Vec::new();
    for result in rows {
        file_infos.push(result.unwrap());
    }
    file_infos
}

///查找非同名文件
pub fn find_non_same_name(source_type: &str, target_type: &str) -> Vec<FileInfo> {
    let sql = format!(
        r#"select source.name,source.path,source.ext
            from source_data source
            where source.name not in (select compare.name from compare_data
            compare where compare.ext = lower('{}'))
              and source.ext = lower('{}')"#,
        target_type, source_type
    );
    println!("non same sql {}", &sql);
    let conn = get_conn();
    let mut stat = conn.prepare(&sql).unwrap();
    let rows = stat
        .query_map(params![target_type, source_type], |row| {
            Ok(FileInfo {
                name: row.get_unwrap(0),
                path: row.get_unwrap(1),
                ext: row.get_unwrap(2),
            })
        })
        .unwrap();
    let mut file_infos = Vec::new();
    for result in rows {
        file_infos.push(result.unwrap());
    }
    file_infos
}
