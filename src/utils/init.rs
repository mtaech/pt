use egui::FontFamily::{Monospace, Proportional};
use egui::{FontDefinitions, FontId, TextStyle};

///初始化字体
pub fn init_default_font() -> FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    let font = include_bytes!("../../assets/LXGWWenKaiScreen.ttf");
    fonts.font_data.insert(
        "LXGWNeoXiHei".to_owned(),
        egui::FontData::from_owned(Vec::from(font)),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "LXGWNeoXiHei".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("LXGWNeoXiHei".to_owned());
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
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (heading2(), FontId::new(26.0, Proportional)),
        (heading3(), FontId::new(24.0, Proportional)),
        (TextStyle::Body, FontId::new(20.0, Proportional)),
        (TextStyle::Monospace, FontId::new(20.0, Monospace)),
        (TextStyle::Button, FontId::new(20.0, Proportional)),
        (TextStyle::Small, FontId::new(18.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
