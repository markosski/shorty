use eframe::egui::{Ui, CursorIcon};

use crate::config::Config;
use crate::content::Content;

pub struct SheetList {
    pub selected_name: String
}

impl SheetList {
    pub fn init(content: &mut Content, config: &Config) -> SheetList {
        if let Some(name) = config.cached_names.first() {
            content.load(name, config);
            SheetList {
                selected_name: name.clone()
            }
        } else {
            SheetList {
                selected_name: "".to_string()
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui, config: &Config, content: &mut Content) {
        ui.label("Available cheat sheets");
        // ui.add(egui::Separator::default());

        for name in &config.cached_names {
            if ui.selectable_label(self.selected_name.eq(name), name).clicked() {
                content.load(&name, config);
                content.search_term = "".to_string();
                self.selected_name = name.clone();
            }
        }
    }
}