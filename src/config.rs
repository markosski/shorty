use std::fs;
use std::io::Error;

pub const CONFIG_DIR: &str = ".shorty";
pub const CONFIG_CACHE: &str = "cache";

pub struct Config {
    pub cached_names: Vec<String>
}

impl Config {
    pub fn init() -> Result<Config, Error> {
        let home = std::env::var("HOME").unwrap();
        let dir_path = format!("{}/{}", &home, CONFIG_DIR);
        let dir_path_cache = format!("{}/{}/cache", &home, CONFIG_DIR);

        fs::DirBuilder::new().recursive(true).create(&dir_path)?;
        fs::DirBuilder::new().recursive(true).create(&dir_path_cache)?;

        let cache = Config::load_names(&dir_path_cache)?;
        Ok(Config {cached_names: cache})
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
}
