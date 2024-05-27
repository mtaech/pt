use std::fs;
use std::fs::File;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::sync::mpsc::Sender;

use crate::cmd::models::FileInfo;
use crate::utils::db;
use rusqlite::params;

use crate::utils::db::get_conn;

///插入表数据
pub fn insert_dir_data(table_name: &str, dir_path: PathBuf) {
    let base_dir = format!("{}", &dir_path.display());
    let conn = get_conn();
    conn.execute(format!("delete from {}", &table_name).as_str(), ())
        .expect("");
    insert_file_info(table_name, dir_path, &base_dir);
}

fn insert_file_info(table_name: &str, dir_path: PathBuf, base_dir: &str) {
    let conn = get_conn();
    if dir_path.exists() {
        for entry in fs::read_dir(&dir_path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                insert_file_info(table_name, entry_path.clone(), base_dir);
            }
            if entry_path.is_file() {
                let path = &entry_path.to_str().unwrap();
                let name = get_file_name_without_ext(&entry_path, &dir_path, base_dir);
                let ext = get_file_ext(&entry_path).to_owned();
                let sql = format!(
                    "INSERT INTO {} (name,path,ext) VALUES (?1,?2,?3)",
                    table_name
                );
                conn.execute(&sql, params![name, path, ext]).expect("");
            }
        }
    }
}

///获取除去的后缀名的文件名
fn get_file_name_without_ext(path: &PathBuf, dir_path: &PathBuf, base_dir: &str) -> String {
    let file_name = format!("{}", &path.display());
    let replace_path = format!("{}", base_dir);
    let file_name = file_name.replace(&replace_path, "");
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
        println!("target dir {}", target_dir.as_str());
        for file_info in file_vec {
            let target_dir = PathBuf::from(&target_dir);
            if !target_dir.exists() {
                fs::create_dir_all(&target_dir).expect("");
            }
            let target_path =
                target_dir.join(format!("{}.{}", &file_info.name, upper(&file_info.ext)));
            println!("tar path {}",&target_path.display());
            /*println!(
                "sour file:{} ,target path :{}",
                &file_info.path,
                &target_path.display()
            );*/
            let dir = &target_path.parent().expect("");
            if !dir.exists() {
                fs::create_dir_all(&dir).expect("");
            }
            match fs::copy(&file_info.path, target_path) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}",e)
                }
            };
        }
    }
}
fn upper(string: &str) -> String {
    string.to_uppercase()
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
            match fs::remove_file(&file_info.path) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}",e);
                }
            };
        }
    }
}

pub fn move_same(target_dir: &String, source_type: &str, target_type: &str) {
    let path_vec = db::find_same_name(source_type, target_type);
    copy(&path_vec, &target_dir.clone());
    delete(&path_vec);
}
pub fn move_none_same(target_dir: &String, source_type: &str, target_type: &str) {
    let path_vec = db::find_non_same_name(source_type, target_type);
    copy(&path_vec, &target_dir.clone());
    delete(&path_vec);
}
