use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::fs::{self, DirEntry};
use linked_hash_map::LinkedHashMap;
use eframe::egui::Ui;
use std::str::FromStr;
use std::io::Write;
use std::io::Error;

pub const CONFIG_DIR: &str = ".shorty";
pub const CONFIG_FILE: &str = "config.yml";
pub const CONFIG_CACHE: &str = "cache";
const CONFIG_DEFAULT_CONTENT: &str = "---\n
url: https://github.com/markosski/shorty/\n
names: vim\n
system: MAC\n";

#[derive(PartialEq, Debug)]
pub enum System {
    MAC, LINUX, WINDOWS
}

impl FromStr for System {

    type Err = ();

    fn from_str(input: &str) -> Result<System, Self::Err> {
        match input {
            "MAC"  => Ok(System::MAC),
            "LINUX"  => Ok(System::LINUX),
            "WINDOWS"  => Ok(System::WINDOWS),
            _      => Err(()),
        }
    }
}


pub struct Config {
    pub url: String,
    pub names: String,
    pub cached_names: Vec<String>,
    pub system: System
}

impl Config {
    pub fn init() -> Result<(), Error> {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/{}/{}", &home, CONFIG_DIR, CONFIG_FILE);
        println!("initializing config: {}", &path);
        if let Err(err) = fs::File::open(&path) {
            if let Ok(mut file) = fs::File::create(&path) {
                if let Err(err) = file.write(CONFIG_DEFAULT_CONTENT.as_bytes()) {
                    Err(err)
                } else {
                    Ok(())
                }
            } else {
                Err(err)
            }
        } else {
            Ok(())
        }
    }

    pub fn load_names(path: &String) -> Result<Vec<String>, Error> {
        let mut result = vec![];

        match fs::read_dir(path) {
            Ok(paths) => {
                for path in paths {
                    if let Ok(entry) = path {
                        let path = entry.path();
                        if let Some(ext) = path.extension() {
                            if ext.eq("yml") {
                                println!("{:?}", &path.file_name());
                                let file_name = &path.file_name().map(|s| s.to_str().unwrap()).unwrap().to_string().strip_suffix(".yml").unwrap().to_string();
                                result.push(file_name.clone());
                            }
                        }
                    }
                }
                Ok(result)
            }
            Err(err) => Err(err)
        }
    }

    pub fn read_config(path: &String) -> Result<Config, Error> {
        let home = std::env::var("HOME").unwrap();
        let full_path = format!("{}/{}", home, path);
        println!("{}", full_path);

        let contents = fs::read_to_string(full_path)
        .expect("Should have been able to read the file");

        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let doc = &docs[0];
        println!("doc - {:?}", &doc);

        let cached_names = Config::load_names(&format!("{}/{}/{}/", home, CONFIG_DIR, CONFIG_CACHE))?;
        let first_cached_name = cached_names[0].clone();
        println!("{:?}", &cached_names);

        Ok(Config {
            url: doc["url"].as_str().map(|s| s.to_string()).unwrap_or("".to_string()),
            names: doc["names"].as_str().map(|s| s.to_string()).unwrap_or("".to_string()),
            cached_names: cached_names,
            system: doc["system"].as_str().map(|s| System::from_str(s).unwrap()).unwrap_or(System::MAC)
        })
    }

    pub fn write_config(path: &String, config: &Config) -> () {
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);

        let mut map = LinkedHashMap::new();
        map.insert(Yaml::String("url".to_string()), Yaml::String(config.url.clone()));
        map.insert(Yaml::String("names".to_string()), Yaml::String(config.names.clone()));
        map.insert(Yaml::String("system".to_string()), Yaml::String(format!("{:?}", &config.system)));

        let doc = &Yaml::Hash(map);

        emitter.dump(&doc).unwrap();

        let home = std::env::var("HOME").unwrap();
        let full_path = format!("{}/{}", home, path);

        println!("{}", out_str);
        fs::write(full_path, out_str).unwrap();
    }
}
