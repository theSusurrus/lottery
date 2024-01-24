use config::Config;
use std::{net::{SocketAddrV4, Ipv4Addr}, collections::HashMap, str::FromStr};

const CONFIG_PATH : &str = "config.toml";

pub struct LotteryConfig {
    pub socket : SocketAddrV4
}

impl LotteryConfig {
    pub fn new() -> LotteryConfig {
        /* Create a config builder, based on file from CONFIG_PATH */
        let config_builder: Config = Config::builder()
            .add_source(config::File::with_name(CONFIG_PATH))
            .build()
            .unwrap();
        let config: HashMap<String, String> = config_builder
            .try_deserialize::<HashMap<String, String>>()
            .expect("Invalid config file");

        /* get address from config */
        let address: Ipv4Addr = Ipv4Addr::from_str(
            config
                .get("address")
                .expect("No address found in {CONFIG_PATH}")
            ).expect("Invalid IP address");

        /* get port from config */
        let port = u16::from_str(
            config
                .get("port")
                .expect("No port found in {CONFIG_PATH}")
            ).expect("Invalid port value");

        LotteryConfig {
            socket : SocketAddrV4::new(address, port)
        }
    }
}
