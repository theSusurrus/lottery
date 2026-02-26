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
    pub name_source : Option<String>,
    pub name_source_type : LotteryConfigSourceType,
}

impl LotteryConfig {

    pub fn new(config_path: &str) -> LotteryConfig {
        let config_builder: Config =
            Config::builder().add_source(config::File::with_name(config_path))
                             .build()
                             .unwrap();
        let config_map: HashMap<String, String> =
            config_builder.try_deserialize::<HashMap<String, String>>()
                          .expect("Invalid config file");

        let name_source_type: LotteryConfigSourceType = 
            config_map.get(CONFIG_NAME_SOURCE_TYPE)
                      .expect("Name source type not configured")
                      .as_str()
                      .into();
        let name_source: Option<String> = 
            match name_source_type {
                LotteryConfigSourceType::FILE => 
                    Some(config_map.get(CONFIG_NAME_SOURCE)
                                .expect("Name source not configured")
                                .to_string()),
                LotteryConfigSourceType::PASTE =>
                    None,
                _ => panic!()
            };

        LotteryConfig {
            name_source,
            name_source_type
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::*;

    #[test]
    fn config_file_test() {
        let config = LotteryConfig::new("config_file.toml");

        assert_eq!(config.name_source.is_some(), true);
        assert_eq!(config.name_source.unwrap(), "src/test_names.htm");
        assert_eq!(config.name_source_type == LotteryConfigSourceType::FILE, true);
    }

    #[test]
    fn config_paste_test() {
        let config = LotteryConfig::new("config_paste.toml");

        assert_eq!(config.name_source, None);
        assert_eq!(config.name_source_type == LotteryConfigSourceType::PASTE, true);
    }
}
