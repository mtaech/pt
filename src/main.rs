#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::cell::OnceCell;
use std::io::Cursor;

use egui::IconData;
use image::ImageFormat;
use rusqlite::Connection;
use pt_plus::app::PtApp;
use pt_plus::db;


pub const DB: OnceCell<Connection> = OnceCell::new();

fn main() -> eframe::Result<()> {
    env_logger::init();
    db::init_db();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 1000.0])
            .with_min_inner_size([1000.0, 750.0])
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "PT",
        native_options,
        Box::new(|cc| Box::new(PtApp::new(cc))),
    )
}
fn load_icon() -> IconData {
    let bytes = include_bytes!("../assets/favicon.png");
    let (icon_rgba, icon_width, icon_height) = {
        let data: Vec<u8> = bytes.into();
        let mut reader = image::io::Reader::new(Cursor::new(data));
        reader.set_format(ImageFormat::Png);
        let image = reader.decode().unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
