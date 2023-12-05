use std::fmt::Display;
use std::path::PathBuf;
use eframe::CreationContext;

use eframe::epaint::Color32;
use eframe::glow::Context;
use egui::{Label, ProgressBar, Ui, Widget};
use egui::accesskit::Role::ProgressIndicator;
use egui::CursorIcon::Progress;
use egui_modal::Modal;
use image::load;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use crate::cmd::operator::insert_dir_data;

use crate::views::models::{FileOperate, FileTypes};

#[derive(Default,Serialize, Deserialize)]
pub struct Manipulation {
    ///操作目录
    pub main_dir: String,
    ///对比目录
    pub compare_dir: String,
    ///操作目标目录
    pub target_dir: String,
    ///操作文件格式
    #[serde(skip)]
    pub main_suffix: FileTypes,
    ///目标对比格式
    #[serde(skip)]
    pub compare_suffix: FileTypes,
    ///消息列表
    #[serde(skip)]
    pub msg_list: Vec<String>,
    #[serde(skip)]
    operate_type: FileOperate,
}

impl Manipulation {
    pub(crate) fn default() -> Self {
        Default::default()
    }
    pub fn show(&mut self, ui: &mut Ui) {
        let mut modal = loading_modal(ui.ctx());
        ui.horizontal(|ui| {
            ui.label("文件操作目录:");
            ui.set_min_width(500.0);
            if ui.text_edit_singleline(&mut self.main_dir).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.main_dir = path.display().to_string();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("文件操作格式");
            egui::ComboBox::from_label("选择格式")
                .selected_text(format!("{:?}", self.main_suffix))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    for file_type in FileTypes::iter() {
                        ui.selectable_value(
                            &mut self.main_suffix,
                            file_type.clone(),
                            file_type.to_string(),
                        );
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("目标对比目录:");
            ui.set_min_width(500.0);
            let mut raw_clone = self.compare_dir.clone();
            if ui.text_edit_singleline(&mut raw_clone).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.compare_dir = path.display().to_string();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("目标对比格式");
            egui::ComboBox::from_label("选择目标对比格式")
                .selected_text(format!("{:?}", self.compare_suffix))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    for file_type in FileTypes::iter() {
                        ui.selectable_value(
                            &mut self.main_suffix,
                            file_type.clone(),
                            file_type.to_string(),
                        );
                    }
                });
        });
        ui.horizontal(|ui| {
            match self.operate_type {
                FileOperate::Delete => ui.set_visible(false),
                FileOperate::DeleteReserve => ui.set_visible(false),
                _ => ui.set_visible(true),
            }
            ui.label("操作目标目录:");
            ui.set_min_width(500.0);
            let mut target_clone = self.target_dir.clone();
            if ui.text_edit_singleline(&mut target_clone).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.target_dir = path.display().to_string();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("文件操作类型:");
            egui::ComboBox::from_label("选择文件操作类型")
                .selected_text(self.operate_type.to_string())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    for file_operate in FileOperate::iter() {
                        ui.selectable_value(
                            &mut self.operate_type,
                            file_operate.clone(),
                            file_operate.to_string(),
                        );
                    }
                });
        });
        ui.vertical(|ui| {
            let mut text = "";
            match self.operate_type {
                FileOperate::Copy => {
                    text = "对比操作目录与目标对比目录内的文件。\n如果操作目录内的文件目标对比目录内有相同文件名的文件。那么将操作目录内的该文件复制到操作目标目录";
                }
                FileOperate::CopyReserve => {
                    text = "对比操作目录与目标对比目录内的文件。\n如果操作目录内的文件目标对比目录内没有相同文件名的文件，那么将操作目录内的该文件复制到操作目标目录";
                }
                FileOperate::Delete => {
                    text = "对比操作目录与目标对比目录内的文件。\n如果操作目录内的文件在目标对比目录里能找到相同文件名的文件，那么将删除操作目录内的该文件";
                }
                FileOperate::DeleteReserve => {
                    text = "对比操作目录与目标对比目录内的文件。\n如果操作目录内的文件目标对比目录内没有相同文件名的文件，那么将删除操作目录内的该文件";
                }
                FileOperate::Move => {
                    text = "对比操作目录与目标对比目录内的文件。\n如果操作目录内的文件在目标对比目录里能找到相同文件名的文件，那么将移动操作目录内的该文件至操作目标目录";
                }
                FileOperate::MoveReserve => {
                    text = "对比操作目录与目标对比目录内的文件。\n如果操作目录内的文件在目标对比目录里不能找到相同文件名的文件，那么将移动操作目录内的该文件至操作目标目录";
                }
            }

            let rich_text = egui::RichText::new(text)
                .heading().color(Color32::from_rgb(255, 164, 0))
                .underline()
                .size(14.0);
            ui.add_space(5.0);
            ui.add(Label::new(rich_text));
            ui.add_space(5.0);
        });
        ui.horizontal(|ui| {
            let btn = ui.button("开始执行");
            if btn.clicked() {
                ui.set_visible(false);
                println!("开始执行 {:?}", self.operate_type);
                self.msg_list.clear();
                if !&self.main_dir.is_empty() {
                    insert_dir_data("source_data", PathBuf::from(&self.main_dir));
                }
                if !&self.compare_dir.is_empty() {
                    insert_dir_data("target_data", PathBuf::from(&self.compare_dir));
                }
                println!("iam close");
                // modal.close();
            }
        });
    }
}


fn loading_modal(ctx:&egui::Context)->Modal{
    let mut modal = Modal::new(ctx, "loading_modal");

// What goes inside the modal
    modal.show(|ui| {
        // these helper functions help set the ui based on the modal's
        // set style, but they are not required and you can put whatever
        // ui you want inside [`.show()`]
        // modal.title(ui, "loading");
        modal.frame(ui, |ui| {
            ui.spinner();
        });
       /* modal.buttons(ui, |ui| {
            // After clicking, the modal is automatically closed
            if modal.button(ui, "close").clicked() {
            };
        });*/
    });
    modal
}