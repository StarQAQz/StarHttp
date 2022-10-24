mod error;
mod http;
mod thread;
mod config;
mod log;

/*
 * @Description: 程序入口
 * @Author: zhengzetao
 * @Date: 2022-07-05 13:21:18
 */
use std::{
    fs,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    path::Path,
};

use config::{STATIC_RESOURCE_PATH, POOL_SIZE};
use http::handle_connect;
use thread::ThreadPool;

fn main() {
    let path = Path::new(STATIC_RESOURCE_PATH);
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(path) {
            panic!("Initialization failed. Error:{}", e)
        }
    }

    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80);
    let listener = TcpListener::bind(socket_addr).unwrap();
    //创建线程池
    match ThreadPool::new(POOL_SIZE) {
        Ok(pool) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        pool.exec(|| handle_connect(stream));
                    }
                    Err(e) => {
                        eprintln!("Connect Incoming Error:{}", e)
                    }
                }
            }
        }
        Err(e) => panic!("{}", e),
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
