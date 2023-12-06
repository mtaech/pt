use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use crate::cmd::models::FileInfo;
use crate::utils::db;
use rusqlite::params;

use crate::utils::db::get_conn;

///插入表数据
pub fn insert_dir_data(table_name: &str, dir_path: PathBuf) {
    let mut conn = get_conn();
    conn.execute(format!("delete from {}", &table_name).as_str(), ())
        .expect("");
    if dir_path.exists() {
        for entry in fs::read_dir(dir_path).unwrap() {
            let file = entry.unwrap();
            let file_path = file.path();
            let path = &file_path.to_str().unwrap();
            let name = get_file_name_without_ext(&file_path);
            let ext = get_file_ext(&file_path).to_owned();
            let sql = format!(
                "INSERT INTO {} (name,path,ext) VALUES (?1,?2,?3)",
                table_name
            );
            conn.execute(&sql, params![name, path, ext]).expect("");
        }
    }
}

///获取除去的后缀名的文件名
fn get_file_name_without_ext(path: &PathBuf) -> String {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if file_name.contains('.') {
        let idx = file_name.rfind(".").unwrap();
        let file_name = &file_name[..idx];
        return String::from(file_name);
    }
    String::from(file_name)
}

///获取文件名的扩展名
fn get_file_ext(path: &PathBuf) -> String {
    match path.extension() {
        None => "".to_string(),
        Some(ext) => {
            let ext = ext.to_str().unwrap();
            String::from(ext).to_lowercase()
        }
    }
}

pub fn copy_same(target_dir: &String, source_type: &str, target_type: &str) {
    let path_vec = db::find_same_name(source_type, target_type);
    copy(&path_vec, target_dir);
}

pub fn copy_non_same(target_dir: &String, source_type: &str, target_type: &str) {
    let path_vec = db::find_non_same_name(source_type, target_type);
    copy(&path_vec, target_dir);
}

fn copy(file_vec: &Vec<FileInfo>, target_dir: &String) {
    if !file_vec.is_empty() {
        for file_info in file_vec {
            let target_dir = PathBuf::from(target_dir);
            let target_path = target_dir.join(format!("{}.{}", &file_info.name, &file_info.ext));
            fs::copy(&file_info.path, target_path).expect("copy error");
        }
    }
}
pub fn delete_same(source_type: &str, target_type: &str) {
    let path_vec = db::find_same_name(source_type, target_type);
    delete(&path_vec);
}
pub fn delete_none_same(source_type: &str, target_type: &str) {
    let path_vec = db::find_non_same_name(source_type, target_type);
    delete(&path_vec);
}
fn delete(file_vec: &Vec<FileInfo>) {
    if !file_vec.is_empty() {
        for file_info in file_vec {
            fs::remove_file(&file_info.path).expect("delete error");
        }
    }
}

pub fn move_same(target_dir: &String, source_type: &str, target_type: &str) {
    let path_vec = db::find_same_name(source_type, target_type);
    copy(&path_vec, target_dir);
    delete(&path_vec);
}
pub fn move_none_same(target_dir: &String, source_type: &str, target_type: &str) {
    let path_vec = db::find_non_same_name(source_type, target_type);
    copy(&path_vec, target_dir);
    delete(&path_vec);
}
