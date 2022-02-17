// Taken from public eframe template
// https://github.com/emilk/eframe_template

#![forbid(unsafe_code)]

mod app;
use app::TemplateApp;

fn main() {
    let app = TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
