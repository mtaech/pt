use std::fs;
use std::path::PathBuf;

use rusqlite::params;

use crate::utils::db::get_conn;

pub fn insert_dir_data(table_name: &str, dir_path: PathBuf) {
    let mut conn = get_conn();
    conn.execute(format!("delete from {}", &table_name).as_str(), ()).expect("");
    if dir_path.exists() {
        for entry in fs::read_dir(dir_path).unwrap() {
            let file = entry.unwrap();
            let file_path = file.path();
            let path = &file_path.to_str().unwrap();
            let name = &file_path.file_name().unwrap().to_str().unwrap();
            let ext = get_ext(&file_path).to_owned();
            let sql = format!("INSERT INTO {} (name,path,ext) VALUES (?1,?2,?3)", table_name);
            conn.execute(&sql, params![name, path, ext,exif_info.camera_model]).expect("");
        }
    }
}

fn get_ext(path: &PathBuf) -> &str {
    path.extension().unwrap().to_str().unwrap()
}