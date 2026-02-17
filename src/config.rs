use std::collections::HashMap;

use config::Config;

const CONFIG_NAME_SOURCE : &str = "name_source";
const CONFIG_NAME_SOURCE_TYPE : &str = "name_source_type";

#[derive(PartialEq)]
pub enum LotteryConfigSourceType {
    FILE,
    PASTE,
    INVALID
}

impl From<&str> for LotteryConfigSourceType {
    fn from(value: &str) -> Self {
        match value {
            "file" => Self::FILE,
            "paste" => Self::PASTE,
            _ => Self::INVALID
        }
    }
}

pub struct LotteryConfig {
    pub name_source : String,
    pub name_source_type : LotteryConfigSourceType,
}

impl LotteryConfig {

    pub fn new(config_path: &str) -> LotteryConfig {
        /* Create a config builder, based on file from CONFIG_PATH */
        let config_builder: Config = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()
            .unwrap();
        let config_map: HashMap<String, String> = config_builder
            .try_deserialize::<HashMap<String, String>>()
            .expect("Invalid config file");

        LotteryConfig {
            name_source_type : config_map.get(CONFIG_NAME_SOURCE_TYPE)
                                        .expect("Name source not configured")
                                        .as_str()
                                        .into(),
            name_source : config_map.get(CONFIG_NAME_SOURCE)
                                    .expect("Name source not configured")
                                    .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::*;

    #[test]
    fn config_test() {
        let config = LotteryConfig::new("config.toml");

        assert_eq!(config.name_source, "src/test_names.htm");
        assert_eq!(config.name_source_type == LotteryConfigSourceType::FILE, true);
    }
}
