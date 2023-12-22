use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::fs::{self, DirEntry};
use linked_hash_map::LinkedHashMap;
use eframe::egui::Ui;
use std::str::FromStr;

use crate::config::*;
use crate::content::Content;

pub struct Settings {
    show_save: bool,
}

impl Settings {
    pub fn init() -> Settings {
        Settings {
           show_save: false,
        }
    }

    fn save(&mut self, config: &Config) {
        Config::write_config(&format!("{}/{}", CONFIG_DIR, CONFIG_FILE), config);
    }

    pub fn show(&mut self, ui: &mut Ui, content: &mut Content, config: &mut Config) {
        ui.collapsing("Settings", |ui| {
            ui.label("Source url of cheat sheets");
            ui.horizontal(|ui| {
                if ui.text_edit_singleline(&mut config.url).changed() {
                    self.show_save = true;
                }

                if ui.button("Reload").clicked() {
                }
            });

            ui.label("Selected sheets (use comma to separate)");
            if ui.text_edit_singleline(&mut config.names).changed() {
                self.show_save = true;
            }

            ui.label("Operating System");

            ui.horizontal(|ui| {
                if ui.selectable_label(config.system.eq(&System::MAC), format!("{:?}", &System::MAC)).clicked() {
                    config.system = System::MAC;
                    self.show_save = true;
                }
                if ui.selectable_label(config.system.eq(&System::LINUX), format!("{:?}", &System::LINUX)).clicked() {
                    config.system = System::LINUX;
                    self.show_save = true;
                }
                if ui.selectable_label(config.system.eq(&System::WINDOWS), format!("{:?}", &System::WINDOWS)).clicked() {
                    config.system = System::WINDOWS;
                    self.show_save = true;
                }
            });

            ui.add_space(10.0);
            if self.show_save {
                if ui.button("Save").clicked() {
                    self.save(config);
                    self.show_save = false;
                    content.load(&content.content_name.clone(), config)
                }
            }
            ui.add_space(20.0);
        });
    }
}