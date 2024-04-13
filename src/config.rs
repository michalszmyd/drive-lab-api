use std::fs;

use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config.toml";

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database: Database,
}

#[derive(Clone, Debug)]
pub struct Database {
    pub url: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let toml_file_content = fs::read_to_string(CONFIG_FILE_PATH);

        let toml_config: ConfigToml = match toml_file_content {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|_| ConfigToml::empty_conifg()),
            Err(_) => ConfigToml::empty_conifg(),
        };

        AppConfig {
            database: match toml_config.database {
                Some(toml_database) => Database {
                    url: toml_database.url.unwrap_or("".to_string()),
                },
                None => Database {
                    url: "".to_string(),
                },
            },
        }
    }
}

#[derive(Deserialize)]
struct ConfigToml {
    database: Option<ConfigDatabaseToml>,
}

impl ConfigToml {
    pub fn empty_conifg() -> ConfigToml {
        let empty_database_config = ConfigDatabaseToml {
            url: Some("".to_string()),
        };

        ConfigToml {
            database: Some(empty_database_config),
        }
    }
}

#[derive(Deserialize)]
struct ConfigDatabaseToml {
    url: Option<String>,
}
