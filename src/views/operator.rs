use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

use chrono::{Local, Utc};
use eframe::epaint::Color32;
use egui::{Label, TextEdit, Ui, Widget, WidgetInfo};
use egui_extras::{Column, TableBuilder};
use egui_modal::Modal;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::cmd::operator::{
    copy_non_same, copy_same, delete_none_same, delete_same, insert_dir_data, move_none_same,
    move_same,
};
use crate::views::models::{FileOperate, FileTypes};

#[derive( Serialize, Deserialize)]
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
    show_tips: bool,
    done_time: String,
}

impl Manipulation {
    pub(crate) fn init() -> Self {
        Manipulation {
            main_dir: "".to_string(),
            compare_dir: "".to_string(),
            target_dir: "".to_string(),
            main_suffix: Default::default(),
            compare_suffix: Default::default(),
            msg_list: vec![],
            operate_type: Default::default(),
            show_tips: false,
            done_time:"".to_string()
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        let padding = 20.0;
        ui.horizontal(|ui| {
            ui.label("文件操作目录");
            let response = ui.add(
                TextEdit::singleline(&mut self.main_dir)
                    .hint_text("选择文件操作目录")
                    .desired_width(ui.available_width() - padding),
            );
            if response.clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.main_dir = path.display().to_string();
                }
            }
        });
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("文件操作格式");
            egui::ComboBox::from_label("选择格式")
                .selected_text(format!("{:?}", self.main_suffix))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(100.0);
                    for file_type in FileTypes::iter() {
                        ui.selectable_value(
                            &mut self.main_suffix,
                            file_type.clone(),
                            file_type.to_string(),
                        );
                    }
                });
        });
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("目标对比目录:");
            let response = ui.add(
                TextEdit::singleline(&mut self.compare_dir)
                    .hint_text("选择目标对比目录")
                    .desired_width(ui.available_width() - padding),
            );
            if response.clicked() {
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
                    ui.set_min_width(100.0);
                    for file_type in FileTypes::iter() {
                        ui.selectable_value(
                            &mut self.compare_suffix,
                            file_type.clone(),
                            file_type.to_string(),
                        );
                    }
                });
        });
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            match self.operate_type {
                FileOperate::Delete => ui.set_visible(false),
                FileOperate::DeleteReserve => ui.set_visible(false),
                _ => ui.set_visible(true),
            }
            ui.label("操作目标目录:");
            let response = ui.add(
                TextEdit::singleline(&mut self.target_dir)
                    .hint_text("选择操作目标目录")
                    .desired_width(ui.available_width() - padding),
            );
            if response.clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.target_dir = path.display().to_string();
                }
            }
        });
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("文件操作类型:");
            egui::ComboBox::from_label("选择文件操作类型")
                .selected_text(self.operate_type.to_string())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(200.0);
                    for file_operate in FileOperate::iter() {
                        ui.selectable_value(
                            &mut self.operate_type,
                            file_operate.clone(),
                            file_operate.to_string(),
                        );
                    }
                });
        });
        ui.add_space(5.0);

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
            ui.add_space(5.0);


            let rich_text = egui::RichText::new(text)
                .heading().color(Color32::from_rgb(255, 164, 0))
                .underline()
                .size(18.0);
            ui.add_space(5.0);
            ui.add(Label::new(rich_text));
            ui.add_space(5.0);
        });
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            let btn = ui.button("开始执行");
            if btn.clicked() {
                self.execute_operate();
            }
        });
        ui.horizontal(|ui| {
            ui.set_visible(self.show_tips);
            let rich_text = egui::RichText::new(format!("成功执行完成！{}",self.done_time))
                .heading().color(Color32::from_rgb(255, 164, 0))
                .underline()
                .size(18.0);
            ui.label(rich_text);
        });
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        // self.table_ui(ui);
    }
}

impl Manipulation {
    fn execute_operate(&mut self,){

        println!("{:#?}开始执行 {:?}", Local::now(), self.operate_type);
        self.msg_list.clear();
        insert_dir_data("source_data", PathBuf::from(&self.main_dir));
        insert_dir_data("compare_data", PathBuf::from(&self.compare_dir));
        match self.operate_type {
            FileOperate::Copy => {
                copy_same(
                    &self.target_dir,
                    &self.main_suffix.to_string(),
                    &self.compare_suffix.to_string(),
                );
            }
            FileOperate::CopyReserve => {
                copy_non_same(
                    &self.target_dir,
                    &self.main_suffix.to_string(),
                    &self.compare_suffix.to_string(),
                );
            }
            FileOperate::Delete => {
                delete_same(
                    &self.main_suffix.to_string(),
                    &self.compare_suffix.to_string(),
                );
            }
            FileOperate::DeleteReserve => {
                delete_none_same(
                    &self.main_suffix.to_string(),
                    &self.compare_suffix.to_string(),
                );
            }
            FileOperate::Move => {
                move_same(
                    &self.target_dir,
                    &self.main_suffix.to_string(),
                    &self.compare_suffix.to_string(),
                );
            }
            FileOperate::MoveReserve => {
                move_none_same(
                    &self.target_dir,
                    &self.main_suffix.to_string(),
                    &self.compare_suffix.to_string(),
                );
            }
        }
        self.show_tips = true;
        self.done_time = Utc::now().to_rfc3339();
        println!("iam close {:#?}", Local::now());
    }
    fn table_ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let available_height = ui.available_height();
            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::initial(200.0))
                .column(Column::initial(500.0))
                .min_scrolled_height(400.0)
                .max_scroll_height(available_height);
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("序号");
                    });
                    header.col(|ui| {
                        ui.strong("处理文件");
                    });
                })
                .body(|mut body| {
                    let mut index = 1;
                    for msg in &self.msg_list {
                        body.row(40.0, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", index));
                            });
                            row.col(|ui| {
                                ui.label(msg);
                            });
                        });
                        index += 1;
                    }
                });
        });
    }
}
