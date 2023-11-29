use egui::{Ui, WidgetText};
use egui_dock::TabViewer;

#[derive(Debug)]
pub struct TabDef {
    pub title: String,
    pub id: String,
}

pub type Tab = TabDef;

// To define the contents and properties of individual tabs, we implement the `TabViewer`
// trait. Only three things are mandatory: the `Tab` associated type, and the `ui` and
// `title` methods. There are more methods in `TabViewer` which you can also override.
pub struct PtTabsViewer;

impl TabViewer for PtTabsViewer {
    // This associated type is used to attach some data to each tab.
    type Tab = Tab;

    // Returns the current `tab`'s title.
    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title.clone().into()
    }

    // Defines the contents of a given `tab`.
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {:#?}", tab));
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }
}
