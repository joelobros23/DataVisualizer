// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, CreationContext};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Serialize)]
struct AppState {
    // Example state - replace with your actual application state
    name: String,
    age: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            name: "Ferris".to_string(),
            age: 3,
        }
    }
}

#[derive(Default)]
struct DataVisualizerApp {
    state: AppState,
}

impl DataVisualizerApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     if let Some(app_state) = eframe::get_value(storage, eframe::APP_KEY) {
        //         return Self { state: app_state };
        //     }
        // }
        Default::default()
    }

    // Example function to load data from a JSON file (replace with your actual data loading)
    fn load_data_from_json(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let data: AppState = serde_json::from_reader(reader)?;
        self.state = data;

        Ok(())
    }
}

impl App for DataVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Visualizer");

            ui.label(format!("Name: {}", self.state.name));
            ui.label(format!("Age: {}", self.state.age));

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.state.name);
            });

            ui.add(egui::Slider::new(&mut self.state.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.state.age += 1;
            }

            if ui.button("Load Data").clicked() {
                // Example: load data from a json file
                if let Err(e) = self.load_data_from_json("data.json") {
                    eprintln!("Error loading data: {}", e);
                }
            }

            ui.label("Non-interactive plots would go here");
        });
    }

    // Optional: Saving the state before shutdown
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, &self.state);
    // }
}

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Data Visualizer",
        options,
        Box::new(|cc| {
            // This gives you access to app initialization stuff.
            // You can inspect the `cc` to find out about the platform, preferred theme, etc.
            // Also be sure to add the `persistence` feature to your `Cargo.toml` file so the app can remember your position and size.

            Box::new(DataVisualizerApp::new(cc))
        }),
    )
}
