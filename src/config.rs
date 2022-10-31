use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, sync::Once};

//静态目录
pub const STATIC_RESOURCE_PATH: &str = "./static";
//线程池大小
pub const POOL_SIZE: usize = 6;
//时区
pub const TIMEZONE: i32 = 8;
//IP
pub const IP: &str = "127.0.0.1";
//Port
pub const PORT: u16 = 80;

const CONFIG_PATH: &str = "./config.toml";
static ONCE: Once = Once::new();
static mut CONFIG: Option<HashMap<String, ConfValType>> = Option::None;

#[derive(Debug)]
enum ConfValType {
    Text(String),
    Num(isize),
    None,
}
fn init_config() {
    ONCE.call_once(|| {
        parse_config(read_config());
    });
}

fn read_config() -> String {
    let config_path = PathBuf::from(CONFIG_PATH).canonicalize().unwrap();
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
                    //读取字符串
                    let vs: Vec<&str> = v.split("\"").collect();
                    value = ConfValType::Text((*vs.get(1).unwrap()).to_owned());
                } else {
                    //读取数值
                    let vs: Vec<&str> = v.split("#").collect();
                    value = ConfValType::Num((*vs.get(0).unwrap()).parse::<isize>().unwrap());
                }
            }
            config_kv.insert(key.to_owned(), value);
        }
    }
    unsafe {
        CONFIG = Option::Some(config_kv);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_config() {
        init_config();
        unsafe {
            println!("{:?}", CONFIG);
        }
    }
}
