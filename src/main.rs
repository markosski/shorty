mod settings;
mod side;
mod config;
mod content;

use eframe::egui::{self};
use settings::Settings;
use side::SheetList;
use content::Content;
use config::*;

// TODO: implement pulling shortcut definitions from github (using selected names)
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // Load configuration file
    let mut config = match Config::init() {
        Ok(_) => 
            match Config::read_config(&format!("{}/{}", CONFIG_DIR, CONFIG_FILE)) {
                Ok(config) => config,
                Err(err) => {
                    println!("{}", err.to_string());
                    panic!("Problem reading configuration file.");
                }
            },
        Err(err) => {
            println!("{:?}", err.to_string());
            panic!("Problem initializing configuration file.")
        }
    };

    // Prepare all components and initialize state
    let mut settings = Settings::init();
    let mut content = Content::init();
    let mut side = SheetList::init();
    if !config.cached_names.is_empty() {
        side.selected_name = config.cached_names[0].clone();
        content.load(&side.selected_name, &config);
    }

    eframe::run_simple_native("Shorty - your handy shortcut browser", options, move |ctx, _frame| {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            settings.show(ui, &mut content, &mut config);
        });

        egui::SidePanel::left("left").show(ctx, |ui| {
            side.show(ui, &config, &mut content);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            content.show(ui);
        });
    })
}