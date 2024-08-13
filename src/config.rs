use std::collections::HashMap;

use config::Config;

const CONFIG_NAME_SOURCE : &str = "name_source";

pub struct LotteryConfig {
    pub name_source : String,
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
            name_source : config_map.get(CONFIG_NAME_SOURCE)
                                    .expect("Name source not configured")
                                    .to_string(),
        }
    }
}
