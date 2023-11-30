use std::fs;
use std::path::PathBuf;

use egui::FontFamily::{Monospace, Proportional};
use egui::{Color32, FontDefinitions, FontId, TextStyle, Visuals};
use egui::Shape::Path;
use rusqlite::{Connection, OpenFlags};
use sled::Db;

///初始化字体
pub fn init_default_font() -> FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    let font = include_bytes!("../assets/LXGWWenKaiScreen.ttf");
    fonts.font_data.insert(
        "LXGWWenKaiScreen".to_owned(),
        egui::FontData::from_owned(Vec::from(font)),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "LXGWWenKaiScreen".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("LXGWWenKaiScreen".to_owned());
    fonts
}

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

///初始化字体样式
pub fn configure_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, Monospace)),
        (TextStyle::Button, FontId::new(14.0, Proportional)),
        (TextStyle::Small, FontId::new(12.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
///初始化基础样式
pub fn configure_context_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let mut visuals = Visuals::light();
    visuals.extreme_bg_color = Color32::WHITE;
    visuals.extreme_bg_color = Color32::WHITE;
    visuals.window_fill = Color32::from_rgb(229, 229, 229);
    style.visuals = visuals;
    style.spacing.item_spacing.y = 10f32;
    ctx.set_style(style);
}
