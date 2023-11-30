use egui_dock::{DockArea, DockState, Style, TabViewer};

use crate::init;
use crate::tabs::{PtTabsViewer, Tab, TabsDef, TabsId};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PtApp {
    #[serde(skip)]
    dock_state: DockState<Tab>,
    #[serde(skip)]
    tabs_viewer: PtTabsViewer,
}

impl Default for PtApp {
    fn default() -> Self {
        let tabs = [
            TabsDef {
                title: "图片操作".to_string(),
                id: TabsId::Operator,
            },
            TabsDef {
                title: "图片数据".to_string(),
                id: TabsId::Analysis,
            },
        ]
        .into_iter()
        .collect();
        let dock_state = DockState::new(tabs);
        Self {
            dock_state,
            tabs_viewer: PtTabsViewer::default(),
        }
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
        init::configure_context_style(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for PtApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            /*  egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
                ui.add_space(16.0);
            });*/
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            DockArea::new(&mut self.dock_state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut self.tabs_viewer);
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
