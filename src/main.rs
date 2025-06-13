use eframe::{egui, App, CreationContext};

#[derive(Default)]
struct DataVisualizerApp {
    // Add any application state here
}

impl App for DataVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Visualizer");
            ui.label("Welcome to the Data Visualizer!");
            // Add UI elements here
        });
    }
}

impl DataVisualizerApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // }

        Default::default()
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Data Visualizer",
        options,
        Box::new(|cc| Box::new(DataVisualizerApp::new(cc))),
    )
}
