
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;


use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExifInfo {
    pub camera_model: String,
    pub len_model: String,
    pub focal_length: String,
}

pub fn get_exif(path: &PathBuf) -> ExifInfo {
    let output_result = Command::new("exiftool").arg(path).output();
    if output_result.is_ok() {
        let output = output_result.unwrap();
        let mut exif_map: HashMap<String, String> = HashMap::new();
        if output.status.success() {
            let result = String::from_utf8(output.stdout);
            if result.is_ok() {
                let exif_lines = result.unwrap();
                for line in exif_lines.lines() {
                    let info = get_exif_info(line);
                    if !exif_map.contains_key(&info.0) {
                        exif_map.insert(info.0, info.1);
                    }
                }
            }
        }
        return map_to_model(exif_map);
    }
    ExifInfo::default()
}
fn map_to_model(exif_map: HashMap<String, String>) -> ExifInfo {
    ExifInfo {
        camera_model: exif_map
            .get("Camera Model Name")
            .map_or("".to_string(), |val| String::from(val)),
        len_model: exif_map
            .get("Lens ID")
            .map_or("".to_string(), |val| String::from(val)),
        focal_length: exif_map
            .get("Focal Length")
            .map_or("".to_string(), |val| String::from(val)),
    }
}
fn get_exif_info(info: &str) -> (String, String) {
    let index = info.find(":").unwrap();
    let key = String::from(&info[..index]);
    let val = String::from(&info[index + 1..]);
    (String::from(key.trim()), String::from(val.trim()))
}
