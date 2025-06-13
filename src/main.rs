use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use plotters::prelude::*;

mod chart_renderer;

#[derive(Deserialize, Serialize, Debug)]
struct DataPoint {
    x: f64,
    y: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    file_path: String,
    chart_type: ChartType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
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

struct DataVisualizerApp {
    config: Config,
    data: Vec<DataPoint>,
    error_message: Option<String>,
}

impl Default for DataVisualizerApp {
    fn default() -> Self {
        Self {
            config: Config {
                file_path: "data.json".to_string(),
                chart_type: ChartType::Scatter,
            },
            data: Vec::new(),
            error_message: None,
        }
    }
}

impl DataVisualizerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts.

        let mut app = DataVisualizerApp::default();
        app.load_data_and_config();  // Load data immediately after creation
        app
    }

    fn load_data_and_config(&mut self) {
        if let Err(e) = self.load_config() {
            self.error_message = Some(format!("Error loading config: {}", e));
        } else if let Err(e) = self.load_data() {
            self.error_message = Some(format!("Error loading data: {}", e));
        } else {
            self.error_message = None;
        }
    }

    fn load_config(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open("config.json")?;
        let reader = BufReader::new(file);
        self.config = serde_json::from_reader(reader)?;
        Ok(())
    }

    fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        let file_path = &self.config.file_path;
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        // Check the file extension to determine parsing method
        if file_path.ends_with(".json") {
            self.data = serde_json::from_reader(reader)?;
        } else if file_path.ends_with(".csv") {
            let mut csv_reader = csv::Reader::from_reader(reader);
            self.data = csv_reader
                .deserialize()
                .filter_map(Result::ok)
                .collect();
        } else {
            return Err(From::from("Unsupported file format.  Use .json or .csv"));
        }
        Ok(())
    }

    fn render_chart(&self, ui: &mut egui::Ui) {
        let chart_type = self.config.chart_type;
        let data = &self.data;

        let frame = Frame::canvas(&ui.style());
        frame.show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size_fixed(), egui::Sense::hover());
            let rect = response.rect;

            let mut plot_buffer = vec![0u8; rect.width() as usize * rect.height() as usize * 4];

            {
                let root = BitMapBackend::with_buffer(&mut plot_buffer, (rect.width() as u32, rect.height() as u32)).into_drawing_area();
                root.fill(&WHITE).unwrap();

                if data.is_empty() {
                    root.print(
                        (rect.width() as i32 / 2, rect.height() as i32 / 2),
                        &TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK),
                        &"No data to display",
                    ).unwrap();
                } else {

                    let mut chart_builder = ChartBuilder::on(&root)
                        .margin(20)
                        .x_label_area_size(30)
                        .y_label_area_size(30);

                    let (min_x, max_x, min_y, max_y) = data.iter().fold((f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY), |(min_x, max_x, min_y, max_y), point| {
                        (min_x.min(point.x), max_x.max(point.x), min_y.min(point.y), max_y.max(point.y))
                    });

                    let mut chart = chart_builder
                        .build_cartesian_2d(min_x..max_x, min_y..max_y).unwrap();

                    chart.configure_mesh().draw().unwrap();

                    match chart_type {
                        ChartType::Scatter => {
                            for point in data {
                                chart.draw_series(std::iter::once(Circle::new((point.x, point.y), 5, ShapeStyle::from(&RED).filled()))).unwrap(); // Adjust radius as needed
                            }
                        }
                        ChartType::Line => {
                            let data_series: Vec<(f64, f64)> = data.iter().map(|p| (p.x, p.y)).collect();
                            chart.draw_series(LineSeries::new(data_series, &BLUE)).unwrap();
                        }
                        ChartType::Bar => {
                            let bar_width = (max_x - min_x) / data.len() as f64;
                            for point in data {
                                chart.draw_series(std::iter::once(Rectangle::new([(point.x - bar_width / 2.0), 0.0], [(point.x + bar_width / 2.0), point.y], ShapeStyle::from(&GREEN).filled()))).unwrap();
                            }
                        }
                    }
                }
            }

            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [rect.width() as usize, rect.height() as usize],
                &plot_buffer,
            );
            let texture_handle = ui.ctx().load_texture("plot_texture", color_image);
            painter.image(
                texture_handle.id(),
                rect,
                egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        });
    }
}

impl App for DataVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Visualizer");

            ui.horizontal(|ui| {
                ui.label("Data File:");
                ui.text_edit_singleline(&mut self.config.file_path);
                if ui.button("Load Data").clicked() {
                    self.load_data_and_config();
                }
            });

            egui::ComboBox::from_label("Chart Type")
                .selected_text(format!("{}", self.config.chart_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Scatter, "Scatter Plot");
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Line, "Line Graph");
                    ui.selectable_value(&mut self.config.chart_type, ChartType::Bar, "Bar Chart");
                });

            if let Some(err) = &self.error_message {
                ui.label(egui::RichText::new(err).color(egui::Color32::RED));
            } else {
                self.render_chart(ui);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Data Visualizer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(DataVisualizerApp::new(cc))
        }),
    )
}
