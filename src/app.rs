use crate::init;
use crate::tabs::{PtTabsViewer, Tab, TabDef};
use egui::{Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PtApp {
    #[serde(skip)] // This how you opt-out of serialization of a field
    dock_state: DockState<Tab>,
}

impl Default for PtApp {
    fn default() -> Self {
        let tabs = [
            TabDef {
                title: "图片操作".to_string(),
                id: "img_operate".to_string(),
            },
            TabDef {
                title: "图片数据".to_string(),
                id: "img_data".to_string(),
            },
        ]
        .into_iter()
        .collect();
        let dock_state = DockState::new(tabs);
        Self { dock_state }
    }
}

impl PtApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        /*   if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }*/
        let my_font = init::init_default_font();
        cc.egui_ctx.set_fonts(my_font);
        init::configure_text_styles(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for PtApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            /*  egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
                ui.add_space(16.0);
            });*/
            egui::widgets::global_dark_light_mode_buttons(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            DockArea::new(&mut self.dock_state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut PtTabsViewer);
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// Here is a simple example of how you can manage a `DockState` of your application.
