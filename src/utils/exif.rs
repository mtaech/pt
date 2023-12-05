use std::fs;
use std::path::PathBuf;

use exif::{Exif, In, Tag};
use serde::{Deserialize, Serialize};

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct ExifInfo {
    pub camera_model: String,
    pub len_model: String,
    pub focal_length: String,
}

pub fn get_exif(path: &PathBuf) -> ExifInfo {
    println!("file path:{:?}",&path);
    let file = fs::File::open(&path).expect("get file error");
    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    match exif_reader.read_from_container(&mut bufreader) {
        Ok(exif) => {
            let camera_model = get_exif_val(Tag::Model, In::PRIMARY, &exif, false);
            let len_model = get_exif_val(Tag::LensModel, In::PRIMARY, &exif, false);
            let focal_length = get_exif_val(Tag::FocalLength, In::PRIMARY, &exif, false);
            return ExifInfo {
                camera_model,
                len_model,
                focal_length,
            }
        }
        Err(_) => ExifInfo::default()
    }
}

fn get_exif_val(tag: Tag, ifd_num: In, exif: &Exif, with_unit: bool) -> String {
    exif.get_field(tag, ifd_num).map_or("".to_string(), |field| {
        let val;
        if with_unit {
            val = field.display_value().with_unit(&*exif).to_string();
        } else {
            val = field.display_value().to_string();
        }
        val.trim_matches('"').to_string()
    })
}