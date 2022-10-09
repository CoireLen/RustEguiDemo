#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::{Context,FontDefinitions,FontData,FontFamily};
use core::include_bytes;
fn main() {
    let mut options = eframe::NativeOptions::default();

    eframe::run_native(
        "学习",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn new() -> Self {
        Self {
            name: "人物名称".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("my_font".to_owned(),
        FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());
        ctx.set_fonts(fonts);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("我的Egui应用");
            ui.horizontal(|ui| {
                ui.label("你的名字: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("年龄"));
            if ui.button("点击年龄+1").clicked() {
                self.age += 1;
            }
            ui.label(format!("你好 '{}', 年龄 {}", self.name, self.age));
        });
    }
}