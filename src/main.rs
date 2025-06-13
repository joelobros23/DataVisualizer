use eframe::{egui, App, Frame, NativeOptions};
use egui::plot::{Line, Plot, Value, Values};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "DataVisualizer",
        native_options,
        Box::new(|cc| Box::new(DataVisualizer::new(cc))),
    );
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataPoint {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataSet {
    name: String,
    points: Vec<DataPoint>,
}

struct DataVisualizer {
    data_sets: Vec<DataSet>,
    file_path: String,
    chart_type: ChartType,
    x_axis_label: String,
    y_axis_label: String,
    filter_value: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ChartType {
    Line,
    Scatter,
}

impl std::fmt::Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartType::Line => write!(f, "Line"),
            ChartType::Scatter => write!(f, "Scatter"),
        }
    }
}

impl DataVisualizer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        DataVisualizer {
            data_sets: Vec::new(),
            file_path: String::new(),
            chart_type: ChartType::Line,
            x_axis_label: "X".to_string(),
            y_axis_label: "Y".to_string(),
            filter_value: 0.0,
        }
    }

    fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        if self.file_path.ends_with(".json") {
            self.load_json()
        } else if self.file_path.ends_with(".csv") {
            self.load_csv()
        } else {
            Err("Unsupported file type".into())
        }
    }

    fn load_json(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        self.data_sets = serde_json::from_reader(reader)?;
        Ok(())
    }

    fn load_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut data_sets = Vec::new();

        for result in rdr.records() {
            let record = result?;
            // Assuming CSV structure: name, x, y
            if record.len() != 3 {
                continue; // Skip malformed records
            }

            let name = record[0].to_string();
            let x: f64 = record[1].parse().unwrap_or(0.0);
            let y: f64 = record[2].parse().unwrap_or(0.0);

            // Check if the dataset already exists
            if let Some(data_set) = data_sets.iter_mut().find(|ds| ds.name == name) {
                data_set.points.push(DataPoint { x, y });
            } else {
                // Create a new dataset
                data_sets.push(DataSet {
                    name: name.clone(),
                    points: vec![DataPoint { x, y }],
                });
            }
        }
        self.data_sets = data_sets;
        Ok(())
    }
}

impl App for DataVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Settings");

            ui.horizontal(|ui| {
                ui.label("File:");
                ui.text_edit_singleline(&mut self.file_path);
                if ui.button("Load").clicked() {
                    if let Err(e) = self.load_data() {
                        eprintln!("Error loading data: {}", e);
                    }
                }
            });

            egui::ComboBox::from_label("Chart Type")
                .selected_text(format!("{}", self.chart_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.chart_type, ChartType::Line, "Line");
                    ui.selectable_value(&mut self.chart_type, ChartType::Scatter, "Scatter");
                });

            ui.horizontal(|ui| {
                ui.label("X Axis Label:");
                ui.text_edit_singleline(&mut self.x_axis_label);
            });

            ui.horizontal(|ui| {
                ui.label("Y Axis Label:");
                ui.text_edit_singleline(&mut self.y_axis_label);
            });

            ui.horizontal(|ui| {
                ui.label("Filter Value:");
                ui.add(egui::DragValue::new(&mut self.filter_value));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let filtered_data_sets: Vec<DataSet> = self
                .data_sets
                .clone()
                .into_iter()
                .map(|ds| DataSet {
                    name: ds.name,
                    points: ds
                        .points
                        .into_iter()
                        .filter(|dp| dp.y > self.filter_value)
                        .collect(),
                })
                .collect();

            let plot = Plot::new("data_plot")
                .view_aspect(2.0)
                .x_axis_label(self.x_axis_label.clone())
                .y_axis_label(self.y_axis_label.clone());

            plot.show(ui, |plot_ui| {
                for data_set in &filtered_data_sets {
                    let values: Values = data_set
                        .points
                        .iter()
                        .map(|dp| Value { x: dp.x, y: dp.y })
                        .collect();

                    match self.chart_type {
                        ChartType::Line => {
                            let line = Line::new(values).name(data_set.name.clone());
                            plot_ui.line(line);
                        }
                        ChartType::Scatter => {
                            let points: Vec<[f64; 2]> = data_set
                                .points
                                .iter()
                                .map(|dp| [dp.x, dp.y])
                                .collect();
                            plot_ui.points(plotters::egui_plot::Points::new(points).name(data_set.name.clone()));
                        }
                    }
                }
            });
        });
    }
}
