use eframe::egui::{Ui, CursorIcon};

use crate::config::Config;
use crate::content::Content;

pub struct SheetList {
    pub selected_name: String
}

impl SheetList {
    // attempt to load content on start
    pub fn init() -> SheetList {
        SheetList {
            selected_name: "".to_string()
        }
    }

    pub fn show(&mut self, ui: &mut Ui, config: &Config, content: &mut Content) {
        ui.label("Available cheat sheets");
        let mut sorted_cached_names = config.cached_names.clone();
        sorted_cached_names.sort();

        for name in &sorted_cached_names {
            if ui.selectable_label(self.selected_name.eq(name), name).clicked() {
                content.load(&name, config);
                content.search_term = "".to_string();
                self.selected_name = name.clone();
            }
        }
    }
}