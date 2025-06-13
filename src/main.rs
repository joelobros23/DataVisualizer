// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, Frame, NativeOptions};

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Data Visualizer",
        native_options,
        Box::new(|cc| Box::new(DataVisualizerApp::new(cc))),
    );
}

#[derive(Default)]
struct DataVisualizerApp {}

impl DataVisualizerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts.
        // Load theme data from local storage.
        Self::default()
    }
}

impl App for DataVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Visualizer");
            ui.label("Welcome to the Data Visualizer!");
        });
    }
}
