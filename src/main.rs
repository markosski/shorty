mod side;
mod config;
mod content;

use eframe::egui::{self};
use side::SheetList;
use content::Content;
use config::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // Load configuration file
    let config = match Config::init() {
        Ok(c) => c,
        Err(err) => {
            println!("{:?}", err.to_string());
            panic!("Problem reading configuration files.")
        }
    };

    // Prepare all components and initialize state
    let mut content = Content::init();
    let mut side = SheetList::init();
    if !config.cached_names.is_empty() {
        side.selected_name = config.cached_names[0].clone();
        content.load(&side.selected_name);
    }

    eframe::run_simple_native("Shorty - your handy shortcut browser", options, move |ctx, _frame| {
        egui::SidePanel::left("left").show(ctx, |ui| {
            side.show(ui, &config, &mut content);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            content.show(ui);
        });
    })
}