use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;


fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Data Visualizer",
        native_options,
        Box::new(|cc| Box::new(DataVisualizer::new(cc))),
    );
}


#[derive(Deserialize, Serialize, Debug, Clone)]
struct DataPoint {
    x: f64,
    y: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DataSet {
    name: String,
    data: Vec<DataPoint>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    data_sets: Vec<DataSet>,
}


struct DataVisualizer {
    config: Config,
}

impl DataVisualizer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = DataVisualizer::load_config("config.json").unwrap_or_else(|_| {
            println!("Failed to load config.json, creating a default one.");
            Config {
                data_sets: vec![
                    DataSet {
                        name: "Example Data".to_string(),
                        data: vec![
                            DataPoint { x: 1.0, y: 2.0 },
                            DataPoint { x: 2.0, y: 3.0 },
                            DataPoint { x: 3.0, y: 1.0 },
                        ],
                    },
                ],
            }
        });
        
        Self {
            config,
        }
    }

    fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

impl App for DataVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Visualizer");

            for data_set in &self.config.data_sets {
                ui.label(format!("Dataset: {}", data_set.name));
                for point in &data_set.data {
                    ui.label(format!("x: {}, y: {}", point.x, point.y));
                }
                ui.separator();
            }

            if ui.button("Reload Config").clicked() {
                match DataVisualizer::load_config("config.json") {
                    Ok(config) => self.config = config,
                    Err(e) => println!("Error reloading config: {}", e),
                }
            }
        });
    }
}
