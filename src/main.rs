// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, NativeOptions, App};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use plotters::prelude::*;
use std::path::Path;

// Define a custom error type for data loading.
#[derive(Debug)]
enum DataError {
    FileError(std::io::Error),
    CsvError(csv::Error),
    SerdeError(serde_json::Error),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::FileError(e) => write!(f, "File error: {}", e),
            DataError::CsvError(e) => write!(f, "CSV error: {}", e),
            DataError::SerdeError(e) => write!(f, "Serde JSON error: {}", e),
        }
    }
}

impl std::error::Error for DataError {}

impl From<std::io::Error> for DataError {
    fn from(e: std::io::Error) -> Self {
        DataError::FileError(e)
    }
}

impl From<csv::Error> for DataError {
    fn from(e: csv::Error) -> Self {
        DataError::CsvError(e)
    }
}

impl From<serde_json::Error> for DataError {
    fn from(e: serde_json::Error) -> Self {
        DataError::SerdeError(e)
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    x: f64,
    y: f64,
}


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ChartType {
    Scatter,
    Line,
    Bar,
}

impl std::fmt::Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartType::Scatter => write!(f, "Scatter"),
            ChartType::Line => write!(f, "Line"),
            ChartType::Bar => write!(f, "Bar"),
        }
    }
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DataVisualizerApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member:
    #[serde(skip)]
    value: f32,

    #[serde(skip)] // Don't serialize loaded data.  Reload on startup.
    data: Vec<DataPoint>,

    selected_chart_type: ChartType,
}

impl Default for DataVisualizerApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            data: Vec::new(),
            selected_chart_type: ChartType::Scatter,
        }
    }
}

impl DataVisualizerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of the egui app.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn load_data_from_csv(&mut self, file_path: &str) -> Result<(), DataError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = Reader::from_reader(reader);

        self.data.clear();
        for result in csv_reader.deserialize() {
            let record: DataPoint = result?;
            self.data.push(record);
        }
        Ok(())
    }

    fn load_data_from_json(&mut self, file_path: &str) -> Result<(), DataError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        self.data = serde_json::from_reader(reader)?;

        Ok(())
    }
}

impl App for DataVisualizerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just use a `CentralPanel`.

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load CSV").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Err(e) = self.load_data_from_csv(path.to_str().unwrap_or("data.csv")) {
                                eprintln!("Error loading data: {}", e);
                            }
                        }
                    }
                    if ui.button("Load JSON").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Err(e) = self.load_data_from_json(path.to_str().unwrap_or("data.json")) {
                                eprintln!("Error loading data: {}", e);
                            }
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.label("Chart Type:");
            ui.radio_value(&mut self.selected_chart_type, ChartType::Scatter, "Scatter");
            ui.radio_value(&mut self.selected_chart_type, ChartType::Line, "Line");
            ui.radio_value(&mut self.selected_chart_type, ChartType::Bar, "Bar");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.label("\nCheck out the different egui demos:");

            egui::warn_if_debug_build(ui);

            ui.label(format!("Loaded {} data points.", self.data.len()));
            ui.label(format!("Selected chart type: {}", self.selected_chart_type));

            let plot_response = egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(ui.available_size_fixed(egui::Vec2::new(300.0, 200.0)), egui::Sense::hover());

                let to_screen = |point: (f32, f32)| {
                    egui::pos2(
                        response.rect.min.x + point.0,
                        response.rect.max.y - point.1,
                    )
                };

                let mut shapes = vec![];
                if !self.data.is_empty() {
                    let max_x = self.data.iter().map(|dp| dp.x).fold(f64::NEG_INFINITY, f64::max);
                    let max_y = self.data.iter().map(|dp| dp.y).fold(f64::NEG_INFINITY, f64::max);
                    let min_x = self.data.iter().map(|dp| dp.x).fold(f64::INFINITY, f64::min);
                    let min_y = self.data.iter().map(|dp| dp.y).fold(f64::INFINITY, f64::min);

                    let scale_x = (response.rect.width() as f64) / (max_x - min_x);
                    let scale_y = (response.rect.height() as f64) / (max_y - min_y);

                    for dp in &self.data {
                        let x = ((dp.x - min_x) * scale_x) as f32;
                        let y = ((dp.y - min_y) * scale_y) as f32;

                        match self.selected_chart_type {
                            ChartType::Scatter => {
                                shapes.push(egui::Shape::Circle(egui::Shape::Circle { center: to_screen((x, y)), radius: 2.0, fill: egui::Color32::RED, stroke: egui::Stroke::NONE }));
                            }
                            ChartType::Line => {
                                // Implement line chart rendering here.  This will need to connect adjacent data points.
                                // For a proper implementation, you'd need to keep track of the previous point.
                                // This is a simplified example.
                                if self.data.len() > 1 { // Basic demonstration.
                                     let first_dp = &self.data[0];
                                     let first_x = ((first_dp.x - min_x) * scale_x) as f32;
                                     let first_y = ((first_dp.y - min_y) * scale_y) as f32;
                                shapes.push(egui::Shape::line_segment([to_screen((first_x, first_y)), to_screen((x, y))], egui::Stroke{width: 1.0, color: egui::Color32::GREEN}));
                                }
                            }
                            ChartType::Bar => {
                                // Implement bar chart rendering here.
                                let bar_width = (response.rect.width() as f32) / (self.data.len() as f32); // Simple width calc.
                                let bar_x_start = x - bar_width / 2.0;
                                let bar_x_end = x + bar_width / 2.0;
                                let rect = egui::Rect::from_min_max(to_screen((bar_x_start, 0.0)), to_screen((bar_x_end, y)));
                                shapes.push(egui::Shape::Rect(egui::Shape::Rect{rect, fill: egui::Color32::BLUE, stroke: egui::Stroke::NONE, rounding: egui::Rounding::NONE}));
                            }
                        }
                    }
                }
                painter.extend(shapes);
            });

            if plot_response.response.hovered() {
                ui.label("Mouse is over the plot");
            }

        });
    }

    /// Called once to store state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1200.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "DataVisualizer",
        native_options,
        Box::new(|cc| {
            // This gives you access to app state storage *at the start*.
            // You can load data here.
            let app = DataVisualizerApp::new(cc);
            Box::new(app)
        }),
    )
}
