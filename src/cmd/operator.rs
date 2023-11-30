use std::fs;
use std::path::PathBuf;
use diesel::associations::HasTable;

use diesel::RunQueryDsl;
use crate::cmd::models::TargetData;

use crate::db::get_conn;
use crate::schema::source_data::dsl::source_data;

pub fn add_dir_data(table_name:&str,dir_path:PathBuf) {

    let mut conn = get_conn();
    diesel::sql_query(format!("delete from {}",&table_name).as_str()).execute(&mut conn).expect("");
    if dir_path.exists() {
        for entry in fs::read_dir(dir_path).unwrap() {
            let file = entry.unwrap();
            let file_path = file.path();
            let path = &file_path.to_str().unwrap();
            let name = &file_path.file_name().unwrap().to_str().unwrap();
            let ext = get_ext(&file_path);
            let data = TargetData {
                name: String::from(name),
                path: String::from(path),
                ext: Option::from(String::from(ext)),
            };
            diesel::insert_into(source_data::table)
                .values(&data).execute(&mut conn).expect("");
        }
    }
}
fn get_ext(path: &PathBuf)->&str{
    path.extension().unwrap().to_str().unwrap()
}