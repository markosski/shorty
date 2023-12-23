use eframe::egui::{Ui, Grid, TextBuffer};
use egui_extras::{TableBuilder, Column};
use eframe::egui;
use fuzzy_matcher::FuzzyMatcher;
use std::{fs, f32::INFINITY};
use std::io::Error;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::config::*;

#[derive(Clone)]
pub struct Data {
    section_descr: String,
    items: Vec<Item>
}

#[derive(Clone)]
pub struct Item {
    shortcut: String,
    description: String
}

pub struct Content {
    pub search_term: String,
    pub content: Vec<Data>,
    pub content_name: String,
    pub filtered: Vec<Data>
}

impl Content {
    pub fn init() -> Content {
        Content {
            search_term: "".to_string(),
            content: vec!(),
            content_name: "".to_string(),
            filtered: vec!()
        }
    }

    pub fn load(&mut self, selected_name: &String, config: &Config) {
        let system = format!("{:?}", &config.system).to_lowercase();
        let system_index = system.as_str();
        let home = std::env::var("HOME").unwrap();
        let full_path = format!("{}/{}/{}/{}.yml", home, CONFIG_DIR, CONFIG_CACHE, selected_name);

        let contents = fs::read_to_string(&full_path)
            .expect("Should have been able to read the file"); 
        
        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let doc = docs[0].as_vec().unwrap();

        let mut all_data: Vec<Data> = vec![];
        for section in doc {
            let section_descr = section["description"].as_str().unwrap();
            let mut all_items: Vec<Item> = vec![];
            let items = section["items"].as_vec().unwrap();

            for item in items {
                let shortcut_map = &item["shortcut"];
                let shortcut = shortcut_map[system_index].as_str()
                    .or(shortcut_map["default"].as_str()).unwrap();
                let description = &item["description"].as_str().unwrap();

                all_items.push(Item{shortcut: shortcut.to_string(), description: description.to_string()});
            }

            let new_section = Data {
                section_descr: String::from(section_descr),
                items: all_items
            };

            all_data.push(new_section);
        }
        
        self.content_name = selected_name.clone();
        self.content = all_data;
        self.filtered = self.content.clone();
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.heading("Search shortcuts 🔎");
        ui.horizontal(|ui| {
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            );

            let input_search = ui.text_edit_singleline(&mut self.search_term);
            if input_search.changed() {
                self.filtered = self.filter(&self.content);
            }
        });
        ui.add_space(20.0);

        egui::ScrollArea::vertical()
        .show(ui, |ui| {
            // Add a lot of widgets here.
            for (i, datum) in self.filtered.iter().enumerate() {
                ui.add_space(20.0);
                ui.heading(&datum.section_descr);
                ui.end_row();

                ui.push_id(i, |ui| {
                    TableBuilder::new(ui)
                    .striped(true)
                    .vscroll(false)
                    .column(Column::auto_with_initial_suggestion(300.0)
                        .at_least(100.0))
                    .column(Column::remainder())
                    .body(|mut body| {
                        for item in &datum.items {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.style_mut().text_styles.insert(
                                        egui::TextStyle::Body,
                                        egui::FontId::new(16.0, eframe::epaint::FontFamily::Proportional),
                                    );
                                    ui.add(egui::Label::new(&item.shortcut).truncate(true));
                                });
                                row.col(|ui| {
                                    ui.add(egui::Label::new(&item.description).wrap(true));
                                });
                            });
                        }
                    });
                });
            }
        });
    }

    fn filter(&self, all_data: &Vec<Data>) -> Vec<Data> {
        let matcher = SkimMatcherV2::default();
        let mut data_filtered: Vec<Data> = vec![];
        for datum in all_data {
            let mut new_datum = Data {section_descr: datum.section_descr.clone(), items: vec![]};

            for item in &datum.items {
                let new_item = item.clone();
                if self.search_term.is_empty() {
                    new_datum.items.push(new_item);
                }
                else if let Some(_) = matcher.fuzzy_match(&item.description, &self.search_term) {
                    new_datum.items.push(new_item);
                }
            }
            
            if !&new_datum.items.is_empty() {
                data_filtered.push(new_datum);
            }
        }

        println!("filtering completed");

        data_filtered
    }
}