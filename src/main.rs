use eframe::{egui, App, CreationContext};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
struct DataPoint {
    x: f64,
    y: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct DataSet {
    name: String,
    data: Vec<DataPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppState {
    data_sets: Vec<DataSet>,
    selected_data_set: Option<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            data_sets: Vec::new(),
            selected_data_set: None,
        }
    }
}

struct DataVisualizerApp {
    state: AppState,
}

impl DataVisualizerApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                return Self { state };
            }
        }

        Self {
            state: AppState::default(),
        }
    }

    fn load_data_from_csv(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        let mut data: Vec<DataPoint> = Vec::new();

        for result in rdr.deserialize() {
            let record: DataPoint = result?;
            data.push(record);
        }

        self.state.data_sets.push(DataSet {
            name: file_path.to_string(),
            data,
        });

        Ok(())
    }
}

impl App for DataVisualizerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Visualizer");

            if ui.button("Load CSV Data").clicked() {
                // In a real app, you'd use a file dialog here.
                // For now, let's just load a hardcoded file.
                if let Err(e) = self.load_data_from_csv("data.csv") {
                    eprintln!("Error loading data: {}", e);
                }
            }

            ui.horizontal(|ui| {
                ui.label("Select Data Set:");
                let mut selected_index = self.state.selected_data_set.unwrap_or(0);
                let data_set_names: Vec<String> = self
                    .state
                    .data_sets
                    .iter()
                    .map(|ds| ds.name.clone())
                    .collect();

                egui::ComboBox::from_label("Data Set")
                    .selected_text(
                        self.state
                            .selected_data_set
                            .map(|i| data_set_names[i].clone())
                            .unwrap_or_else(|| "None".to_string()),
                    )
                    .show_index(ui, &mut selected_index, data_set_names.len(), |i| {
                        data_set_names[i].clone()
                    });

                self.state.selected_data_set = Some(selected_index);
            });

            if let Some(selected_index) = self.state.selected_data_set {
                if let Some(data_set) = self.state.data_sets.get(selected_index) {
                    ui.label(format!("Displaying data from: {}", data_set.name));

                    // Display first 10 data points for demonstration
                    ui.label("First 10 Data Points:");
                    for (i, point) in data_set.data.iter().take(10).enumerate() {
                        ui.label(format!("{}: x={}, y={}", i + 1, point.x, point.y));
                    }

                    // TODO: Implement Plotters integration here to display the data graphically
                    ui.label("Plot will be displayed here (Plotters integration pending)...");

                    if ui.button("Clear Data").clicked() {
                        self.state.data_sets.clear();
                        self.state.selected_data_set = None;
                    }
                }
            }

            ctx.request_repaint(); // Ensure the UI is redrawn every frame
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Data Visualizer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.image_loaders);
            Box::new(DataVisualizerApp::new(cc))
        }),
    )
}
