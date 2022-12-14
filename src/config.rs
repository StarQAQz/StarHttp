use std::{collections::HashMap, env, fs::File, io::Read, path::PathBuf, sync::Once};

const CONFIG_PATH: &str = "config.toml";
static ONCE: Once = Once::new();
static mut CONFIG: Option<HashMap<String, ConfValType>> = Option::None;

#[derive(Debug, Clone)]
enum ConfValType {
    Text(String),
    Num(isize),
    None,
}

pub struct Config {
    config: HashMap<String, ConfValType>,
}

impl Config {
    pub fn build() -> Config {
        ONCE.call_once(|| {
            parse_config(read_config());
        });
        unsafe {
            match CONFIG.clone() {
                Some(config) => Config { config },
                None => panic!("The initial configuration fails!"),
            }
        }
    }

    fn get_text(&self, key: &str) -> Option<String> {
        match self.config.get(key) {
            Some(config) => match config {
                ConfValType::Text(config) => Some(config.clone()),
                _ => None,
            },
            None => None,
        }
    }

    fn get_num(&self, key: &str) -> Option<isize> {
        match self.config.get(key) {
            Some(config) => match config {
                ConfValType::Num(config) => Some(config.clone()),
                _ => None,
            },
            None => None,
        }
    }
}

fn read_config() -> String {
    let exe_path = PathBuf::from(env::args().nth(0).unwrap());
    let exe_dir = exe_path.parent().unwrap();
    let mut config_path = exe_dir.join(CONFIG_PATH);
    if !config_path.exists() {
        config_path = PathBuf::from(CONFIG_PATH);
    }
    if !config_path.exists() || !config_path.is_file() {
        panic!("The configuration file does not exist!");
    }
    let mut file = File::open(config_path).unwrap();
    let mut config = String::new();
    file.read_to_string(&mut config).unwrap();
    config
}

fn parse_config(config: String) {
    let lines: Vec<&str> = config
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let mut config_kv: HashMap<String, ConfValType> = HashMap::new();
    for line in lines {
        if line.starts_with("#") {
            continue;
        }
        if line.contains("=") && !line.starts_with("=") {
            let kv: Vec<&str> = line.split('=').map(|l| l.trim()).collect();
            let key = *kv.get(0).unwrap();
            let mut value = ConfValType::None;
            if let Some(v) = kv.get(1) {
                if v.starts_with("\"") {
                    //???????????????
                    let vs: Vec<&str> = v.split("\"").collect();
                    value = ConfValType::Text((*vs.get(1).unwrap()).to_owned());
                } else {
                    //????????????
                    let vs: Vec<&str> = v.split("#").collect();
                    let v = (*(vs.get(0).unwrap())).trim();
                    value = ConfValType::Num(v.parse::<isize>().unwrap());
                }
            }
            config_kv.insert(key.to_owned(), value);
        }
    }
    unsafe {
        CONFIG = Option::Some(config_kv);
    }
}

//????????????
static MY_CONFIG_ONCE: Once = Once::new();
static mut MY_CONFIG: Option<MyConfig> = None;
#[derive(Clone)]
pub struct MyConfig {
    pub static_resource_path: String,
    pub index_page_path: String,
    pub page404_path: Option<String>,
    pub page500_path: Option<String>,
    pub thread_pool_size: usize,
    pub timezone: i32,
    pub ip: std::net::Ipv4Addr,
    pub port: u16,
}

impl MyConfig {
    pub fn new() -> MyConfig {
        MY_CONFIG_ONCE.call_once(|| {
            let config = Config::build();
            unsafe {
                MY_CONFIG = Some(MyConfig {
                    static_resource_path: Self::get_static_resource_path(&config),
                    index_page_path: Self::get_index_page_path(&config),
                    page404_path: Self::get_page404_path(&config),
                    page500_path: Self::get_page500_path(&config),
                    thread_pool_size: Self::get_thread_pool_size(&config),
                    timezone: Self::get_timezone(&config),
                    ip: Self::get_ip(&config),
                    port: Self::get_port(&config),
                })
            }
        });
        unsafe {
            match &MY_CONFIG {
                Some(config) => config.clone(),
                None => {
                    panic!("Configuration initialization error, please check the configuration!")
                }
            }
        }
    }
    fn get_static_resource_path(config: &Config) -> String {
        match config.get_text("static_resource_path") {
            Some(static_resource_path) => {
                return static_resource_path;
            }
            None => {
                panic!(
                    "The static resource path is incorrectly configured. Check the configuration."
                )
            }
        };
    }

    fn get_index_page_path(config: &Config) -> String {
        match config.get_text("index_page_path") {
            Some(index_page_path) => {
                return index_page_path;
            }
            None => {
                return "index.html".to_owned();
            }
        };
    }

    fn get_page404_path(config: &Config) -> Option<String> {
        match config.get_text("page404_path") {
            Some(page404_path) => {
                return Some(page404_path);
            }
            None => {
                return None;
            }
        };
    }

    fn get_page500_path(config: &Config) -> Option<String> {
        match config.get_text("page500_path") {
            Some(page500_path) => {
                return Some(page500_path);
            }
            None => {
                return None;
            }
        };
    }

    fn get_thread_pool_size(config: &Config) -> usize {
        match config.get_num("thread_pool_size") {
            Some(thread_pool_size) => thread_pool_size as usize,
            None => panic!(
                "The thread pool configuration is incorrect. Please check the configuration."
            ),
        }
    }

    fn get_timezone(config: &Config) -> i32 {
        match config.get_num("timezone") {
            Some(timezone) => timezone as i32,
            None => {
                panic!("The time zone configuration is incorrect. Please check the configuration.")
            }
        }
    }

    fn get_ip(config: &Config) -> std::net::Ipv4Addr {
        match config.get_text("ip") {
            Some(ip) => match ip.parse::<std::net::Ipv4Addr>() {
                Ok(ip) => ip,
                Err(_) => panic!(
                    "The ip address configuration is incorrect. Please check the configuration."
                ),
            },
            None => {
                panic!("The ip address configuration is incorrect. Please check the configuration.")
            }
        }
    }

    fn get_port(config: &Config) -> u16 {
        match config.get_num("port") {
            Some(port) => port as u16,
            None => panic!("The port configuration is incorrect. Please check the configuration."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config = MyConfig::new();
        println!("{}", config.static_resource_path);
        println!("{}", config.thread_pool_size);
        println!("{}", config.timezone);
        let config = MyConfig::new();
        println!("{}", config.ip);
        println!("{}", config.port);
    }
}
