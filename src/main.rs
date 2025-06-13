// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, Frame, NativeOptions};
use egui::plot::{Line, Plot, Value, Values};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Data Visualizer",
        native_options,
        Box::new(|cc| Box::new(DataVisualizer::new(cc))),
    )
}

#[derive(Deserialize, Serialize, Debug)]
struct DataPoint {
    x: f64,
    y: f64,
}

struct DataVisualizer {
    data: Vec<DataPoint>,
    settings: Settings,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Settings {
    line_color: egui::Color32,
    line_width: f32,
    x_axis_label: String,
    y_axis_label: String,
    title: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            line_color: egui::Color32::BLUE,
            line_width: 2.0,
            x_axis_label: "X".to_string(),
            y_axis_label: "Y".to_string(),
            title: "Data Plot".to_string(),
        }
    }
}

impl DataVisualizer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // Load previous app state (if any).
        let settings = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Settings::default()
        };

        let data = load_data_from_csv("data.csv").unwrap_or(vec![]);

        DataVisualizer {
            data,
            settings,
        }
    }
}

impl App for DataVisualizer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("settings_panel").show_inside(ui, |ui| {
                ui.heading("Settings");
                ui.separator();

                ui.label("Line Color:");
                egui::color_picker::color_edit_button_sr(ui, &mut self.settings.line_color);

                ui.add(egui::Slider::new(&mut self.settings.line_width, 1.0..=5.0).text("Line Width"));

                ui.label("X Axis Label:");
                ui.text_edit_singleline(&mut self.settings.x_axis_label);

                ui.label("Y Axis Label:");
                ui.text_edit_singleline(&mut self.settings.y_axis_label);

                 ui.label("Title:");
                ui.text_edit_singleline(&mut self.settings.title);
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                let values: Vec<Value> = self
                    .data
                    .iter()
                    .map(|dp| Value::new(dp.x, dp.y))
                    .collect();

                let line = Line::new(Values::from_values(values)).color(self.settings.line_color).width(self.settings.line_width);

                Plot::new("data_plot")
                    .title(self.settings.title.clone())
                    .x_axis_label(self.settings.x_axis_label.clone())
                    .y_axis_label(self.settings.y_axis_label.clone())
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }
}

fn load_data_from_csv(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut data: Vec<DataPoint> = Vec::new();

    for result in csv_reader.deserialize() {
        let record: DataPoint = result?;
        data.push(record);
    }

    Ok(data)
}
