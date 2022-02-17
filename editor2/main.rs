// Taken from public eframe template (with light modifications)
// https://github.com/emilk/eframe_template

#![forbid(unsafe_code)]

mod app;
mod config;

use app::ConfigEditor;
use config::TaskConfig;

fn main() {
    let app = ConfigEditor::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
