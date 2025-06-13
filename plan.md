# Project Plan: DataVisualizer

**Description:** A desktop application built with Rust and egui for visualizing various data types (CSV, JSON, custom formats) through interactive charts and graphs.


## Development Goals

- [ ] Set up the basic eframe application boilerplate in src/main.rs.
- [ ] Create a struct to hold the application state, including loaded data, chart type selection, and UI elements.
- [ ] Implement the `eframe::App` trait for the struct.
- [ ] In the `update` method, create a `egui::CentralPanel` with a side panel for settings and a main area for visualization.
- [ ] Implement data parsing functionality in src/data_parser.rs. Support CSV and JSON formats. Display parsing errors in the UI.
- [ ] Create a chart rendering module in src/chart_renderer.rs using `plotters`. Support basic chart types like scatter plots, line graphs, and bar charts.
- [ ] Add UI elements to the side panel for: File selection (CSV, JSON), Chart type selection (dropdown), Axis label configuration, and data filtering (basic).
- [ ] Implement data loading and visualization based on selected file and chart type. Handle different data types and display appropriate charts.
- [ ] Add basic interactivity: Zoom and pan on the chart, data point highlighting on hover.
- [ ] Implement error handling for invalid data or configuration. Display user-friendly error messages.
- [ ] Add a menu bar with 'File' -> 'Open', 'File' -> 'Save', and 'File' -> 'Exit' functionality.
- [ ] Implement saving the current chart as an image (PNG, JPEG).
