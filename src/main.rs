#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::IconData;
use image::ImageFormat;
use rust_embed::{EmbeddedFile, RustEmbed};
use std::io::Cursor;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 300.0])
            .with_min_inner_size([600.0, 220.0])
            .with_icon(load_icon()),
        ..Default::default()
    };
    let assets = Assets::get("favicon.ico");
    if assets.is_some() {}
    eframe::run_native(
        "pt",
        native_options,
        Box::new(|cc| Box::new(pt_plus::PtApp::new(cc))),
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
