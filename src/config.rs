use config::Config;
use std::{collections::HashMap, net::{SocketAddrV4, Ipv4Addr}, str::FromStr};

const CONFIG_PATH : &str = "config.toml";

pub struct LotteryConfig {
    pub socket : SocketAddrV4
}

impl LotteryConfig {
    fn create_socket(map: HashMap<String, String>) -> SocketAddrV4 {
        /* get address from config */
        let address: Ipv4Addr = Ipv4Addr::from_str(
            map
                .get("address")
                .expect("No address found in {CONFIG_PATH}")
            ).expect("Invalid IP address");

        /* get port from config */
        let port = u16::from_str(
            map
                .get("port")
                .expect("No port found in {CONFIG_PATH}")
            ).expect("Invalid port value");
        
        SocketAddrV4::new(address, port)
    }

    pub fn new() -> LotteryConfig {
        /* Create a config builder, based on file from CONFIG_PATH */
        let config_builder: Config = Config::builder()
            .add_source(config::File::with_name(CONFIG_PATH))
            .build()
            .unwrap();
        let config_map: HashMap<String, String> = config_builder
            .try_deserialize::<HashMap<String, String>>()
            .expect("Invalid config file");

        LotteryConfig {
            socket : Self::create_socket(config_map)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_test_localhost() {
        let mut map : HashMap<String, String> = HashMap::new();
        map.insert("port".to_string(), "3000".to_string());
        map.insert("address".to_string(), "127.0.0.1".to_string());

        let socket = LotteryConfig::create_socket(map);

        assert_eq!(socket.ip().octets(), [127, 0, 0, 1]);
        assert_eq!(socket.port(), 3000);
    }

    #[test]
    fn socket_test_lan() {
        let mut map : HashMap<String, String> = HashMap::new();
        map.insert("port".to_string(), "4000".to_string());
        map.insert("address".to_string(), "192.168.1.100".to_string());

        let socket = LotteryConfig::create_socket(map);

        assert_eq!(socket.ip().octets(), [192, 168, 1, 100]);
        assert_eq!(socket.port(), 4000);
    }
}
