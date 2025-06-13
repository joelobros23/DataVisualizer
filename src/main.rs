use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Data Visualizer",
        native_options,
        Box::new(|cc| Box::new(DataVisualizer::new(cc))),
    );
}

#[derive(Deserialize, Serialize, Default)]
pub struct DataVisualizer {
    // Example state (replace with your actual data and settings)
    pub settings: Settings,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Settings {
    pub theme: Theme,
    // Example setting:
    pub show_grid: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

impl DataVisualizer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl App for DataVisualizer {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just use CentralPanel::default().
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("settings_panel").show(ui, |ui| {
                ui.heading("Settings");

                ui.checkbox(&mut self.settings.show_grid, "Show Grid");

                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    if ui.radio(self.settings.theme == Theme::Light, "Light").clicked() {
                        self.settings.theme = Theme::Light;
                    }
                    if ui.radio(self.settings.theme == Theme::Dark, "Dark").clicked() {
                        self.settings.theme = Theme::Dark;
                    }
                });

                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Data Visualization");
                ui.label("Here is where the visualization will be.");
                // Example plot (replace with actual plotting logic)
                egui::widgets::plot::Plot::new("my_plot")
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(egui::widgets::plot::Line::new(
                            vec![[0.0, 0.0], [1.0, 1.0]],
                        ));
                    });
            });
        });
    }

    /// Called once before the first frame. All code that set up the app (load resources, …) should be done here.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut Frame,
        _storage: Option<&dyn eframe::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = storage {
        //     *self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}