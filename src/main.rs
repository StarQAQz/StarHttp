mod config;
mod error;
mod hex;
mod http;
mod log;
mod thread;
mod time;

use std::{
    fs,
    net::{SocketAddrV4, TcpListener},
    path::Path,
};

use config::MyConfig;
use http::handle_connect;
use thread::ThreadPool;

fn main() {
    let config = MyConfig::new();
    let path = Path::new(&config.static_resource_path);
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(path) {
            panic!("Initialization failed. Error:{}", e)
        }
    }
    let socket_addr = SocketAddrV4::new(config.ip, config.port);
    match TcpListener::bind(socket_addr) {
        Ok(listener) => {
            //创建线程池
            match ThreadPool::new(config.thread_pool_size) {
                Ok(pool) => {
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                log_info!("Connect Incoming!");
                                pool.exec(|| handle_connect(stream));
                            }
                            Err(e) => {
                                log_error!("Connect Incoming Error:{}", e)
                            }
                        }
                    }
                }
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => {
            log_error!("Failed to listen to the IP port! Err:{}", e);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{env, io};

    #[test]
    fn test_path() -> io::Result<()> {
        let path = "/hello/world";
        let mut current_path = env::current_dir()?;
        for node in path.split("/") {
            current_path.push(node);
        }
        println!("{}", current_path.display().to_string());
        Ok(())
    }
}
