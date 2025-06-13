use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct DataVisualizerApp {
    // Example data (replace with your actual data)
    data: Vec<(f64, f64)>,
    #[serde(skip)]
    file_path: Option<PathBuf>,
}

impl DataVisualizerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                // Assuming a simple CSV format for now
                let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(reader);
                self.data.clear();
                for result in rdr.records() {
                    if let Ok(record) = result {
                        if record.len() == 2 {
                            if let (Ok(x), Ok(y)) = (record[0].parse::<f64>(), record[1].parse::<f64>()) {
                                self.data.push((x, y));
                            }
                        }
                    }
                }
                self.file_path = Some(path);
            }
        }
    }

    fn save_file(&self) {
        if let Some(path) = &self.file_path {
            if let Ok(file) = File::create(path) {
                let mut writer = BufWriter::new(file);
                for (x, y) in &self.data {
                    if let Err(e) = writeln!(writer, "{},{}", x, y) {
                        eprintln!("Error writing to file: {}", e);
                        return;
                    }
                }
            }
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            self.file_path = Some(path.clone());
            if let Ok(file) = File::create(path) {
                let mut writer = BufWriter::new(file);
                for (x, y) in &self.data {
                    if let Err(e) = writeln!(writer, "{},{}", x, y) {
                        eprintln!("Error writing to file: {}", e);
                        return;
                    }
                }
            }
        }
    }
}

impl App for DataVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button("Save As...").clicked() {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Visualizer");

            // Display data (replace with your actual plotting logic)
            for (x, y) in &self.data {
                ui.label(format!("x: {}, y: {}", x, y));
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Data Visualizer",
        native_options,
        Box::new(|cc| Box::new(DataVisualizerApp::new(cc))),
    )
}
