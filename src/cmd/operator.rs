use std::fs;
use std::path::PathBuf;

use exif::{In, Tag};
use rusqlite::params;
use crate::utils::db::get_conn;
use crate::utils::exif::get_exif;

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
            let exif_info = get_exif(&file_path);
            let sql = format!("INSERT INTO {} (name,path,ext,camera_model,len_model,focal_length) VALUES (?1,?2,?3,?4,?5,?6)", table_name);
            conn.execute(&sql, params![name, path, ext,exif_info.camera_model,
                exif_info.len_model,exif_info.focal_length]).expect("");
        }
    }
}

fn get_ext(path: &PathBuf) -> &str {
    path.extension().unwrap().to_str().unwrap()
}

fn get_model(path: &PathBuf) -> String {
    let file = fs::File::open(path).expect("get file error");
    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut bufreader).expect("get exif error");
    let model = exif.get_field(Tag::Model, In::PRIMARY).map_or("".to_string(), |field| {
        let val = field.display_value().to_string();
        val.trim_matches('"').to_string()
    });
    model
}