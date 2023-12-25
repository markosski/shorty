use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::fs::{self, DirEntry};
use linked_hash_map::LinkedHashMap;
use std::io::Error;

pub const CONFIG_DIR: &str = ".shorty";
pub const CONFIG_FILE: &str = "config.yml";
pub const CONFIG_CACHE: &str = "cache";

pub struct Config {
    pub cached_names: Vec<String>
}

impl Config {
    pub fn init() -> Result<(), Error> {
        let home = std::env::var("HOME").unwrap();
        let dir_path = format!("{}/{}", &home, CONFIG_DIR);
        let dir_path_cache = format!("{}/{}/cache", &home, CONFIG_DIR);

        fs::DirBuilder::new().recursive(true).create(&dir_path)?;
        fs::DirBuilder::new().recursive(true).create(&dir_path_cache)?;
        Ok(())
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

    pub fn read_config() -> Result<Config, Error> {
        let home = std::env::var("HOME").unwrap();
        let full_path = format!("{}/{}/{}", &home, CONFIG_DIR, CONFIG_FILE);
        println!("reading config: {}", full_path);

        let contents = fs::read_to_string(full_path)
        .expect("Should have been able to read the file");

        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let doc = &docs[0];
        println!("doc - {:?}", &doc);

        let cached_names = Config::load_names(&format!("{}/{}/{}/", home, CONFIG_DIR, CONFIG_CACHE))?;
        println!("{:?}", &cached_names);

        Ok(Config {
            cached_names: cached_names,
        })
    }

    pub fn write_config(path: &String, config: &Config) -> () {
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);

        let mut map = LinkedHashMap::new();
        let doc = &Yaml::Hash(map);

        emitter.dump(&doc).unwrap();

        let home = std::env::var("HOME").unwrap();
        let full_path = format!("{}/{}", home, path);

        println!("{}", out_str);
        fs::write(full_path, out_str).unwrap();
    }
}
