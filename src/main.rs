/*
 * @Description: 程序入口
 * @Author: zhengzetao
 * @Date: 2022-07-05 13:21:18
 */
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    path::Path,
};

const STATIC_RESOURCE_PATH: &str = "./static";

struct HttpConnect {
    tcp: TcpStream,
}

impl HttpConnect {
    //连接控制，读取请求并判断请求类型
    fn handle_connect(self) {
        if let Ok(first_line) = self.read_line() {
            if let Some(first_line) = first_line {
                println!("{}", &first_line);
                let mut header = first_line.split_whitespace();
                let request_type = header.next().unwrap();
                let url = header.next().unwrap();
                //分发请求类型处理
                match request_type.to_lowercase().as_str() {
                    "get" => {
                        if let Err(e) = self.get(url) {
                            eprintln!(
                                "The GET request is abnormal. Error reason: {}",
                                e.to_string()
                            );
                            if let Err(e) = self.send_internal_server_error() {
                                eprintln!("Response 500 failed. Error reason: {}", e.to_string());
                            }
                        }
                    }
                    val => {
                        eprintln!("Do not support request type! Request type: {}", val);
                    }
                };
            }
        }
        self.shutdown();
    }

    //GET请求
    fn get(&self, url: &str) -> io::Result<()> {
        //读取剩余请求
        while let Some(_) = self.read_line()? {}
        //解析url，分隔参数
        let mut path = url;
        if url.contains("?") {
            let v: Vec<&str> = url.split("?").collect();
            path = v[0];
        }
        //构建文件路径
        let mut current_path = fs::canonicalize(Path::new(STATIC_RESOURCE_PATH))?;
        for node in path.split("/") {
            current_path.push(node);
        }
        match File::open(current_path) {
            Ok(file) => self.send_ok(file)?,
            Err(_) => self.send_not_found()?,
        }
        Ok(())
    }

    fn send_ok(&self, file: File) -> io::Result<()> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("Content-Type", "text/html;text/html; charset=utf-8\r\n");
        let header = ResponseHeader {
            http_status: HttpStatus::OK,
            params,
        };
        self.send(header, Some(&file))
    }

    fn send_not_found(&self) -> io::Result<()> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("Content-Type", "text/html;text/html; charset=utf-8\r\n");
        let header = ResponseHeader {
            http_status: HttpStatus::NotFound,
            params,
        };
        let html = "<!DOCTYPE html><head><title>404 NOT FOUND</title></head><body><h1>404 NOT FOUND!</h1></body></html>";
        self.send(header, Some(&html.to_string()))
    }

    fn send_internal_server_error(&self) -> io::Result<()> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("Content-Type", "text/html;text/html; charset=utf-8\r\n");
        let header = ResponseHeader {
            http_status: HttpStatus::InternalServerError,
            params,
        };
        let html = "<!DOCTYPE html><head><title>500 INTERNAL SERVER ERROR</title></head><body><h1>500 INTERNAL SERVER ERROR!</h1></body></html>";
        self.send(header, Some(&html.to_string()))
    }

    fn send(&self, header: ResponseHeader, body: Option<&dyn ResponseBody>) -> io::Result<()> {
        let mut tcp = &self.tcp;
        tcp.write_all(header.get().as_bytes())?;
        if let Some(body) = body {
            body.write_in_connect(self)?;
        }
        Ok(())
    }

    fn shutdown(self) {
        if let Err(e) = self.tcp.shutdown(std::net::Shutdown::Both) {
            eprintln!(
                "Failed to shutdown the connection. Error reason: {}",
                e.to_string()
            );
        }
    }

    /*
     * 读取一行数据
     */
    fn read_line(&self) -> io::Result<Option<String>> {
        let mut line = String::new();
        let tcp = &self.tcp;
        for buf in tcp.bytes() {
            let val = buf? as char;
            if val == '\r' {
                let val = tcp.bytes().next().unwrap();
                if val? as char != '\n' {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "read line error: Read only '\\r', no '\\n'",
                    ));
                } else {
                    break;
                }
            } else {
                line.push(val);
            }
        }
        if line.len() > 0 {
            Ok(Some(line))
        } else {
            Ok(None)
        }
    }
}

struct ResponseHeader<'a> {
    http_status: HttpStatus,
    params: HashMap<&'a str, &'a str>,
}

impl ResponseHeader<'_> {
    fn get(self) -> String {
        let mut header = String::new();
        header.push_str(self.http_status.get_http_status());
        for (key, val) in self.params.iter() {
            let param = format!("{}:{}\r\n", key, val);
            header.push_str(&param);
        }
        return header;
    }
}

trait ResponseBody {
    fn write_in_connect(&self, connect: &HttpConnect) -> io::Result<()>;
}

impl ResponseBody for File {
    fn write_in_connect(&self, connect: &HttpConnect) -> io::Result<()> {
        let mut tcp = &connect.tcp;
        let mut buf_reader = BufReader::new(self);
        while buf_reader.fill_buf()?.len() > 0 {
            let size = tcp.write(buf_reader.buffer())?;
            buf_reader.consume(size);
        }
        tcp.flush()?;
        Ok(())
    }
}

impl ResponseBody for String {
    fn write_in_connect(&self, connect: &HttpConnect) -> io::Result<()> {
        let mut tcp = &connect.tcp;
        tcp.write_all(self.as_bytes())?;
        tcp.flush()?;
        Ok(())
    }
}

enum HttpStatus {
    OK,                  //"HTTP/1.0 200 OK\r\n"
    NotFound,            //"HTTP/1.0 400 NOT FOUND\r\n"
    InternalServerError, //"HTTP/1.0 500 INTERNAL SERVER ERROR\r\n"
}

impl HttpStatus {
    fn get_http_status(&self) -> &str {
        match self {
            HttpStatus::OK => "HTTP/1.0 200 OK\r\n",
            HttpStatus::NotFound => "HTTP/1.0 404 NOT FOUND\r\n",
            HttpStatus::InternalServerError => "HTTP/1.0 500 INTERNAL SERVER ERROR\r\n",
        }
    }
}

fn init() -> io::Result<()> {
    let path = Path::new(STATIC_RESOURCE_PATH);
    if !path.exists() {
        fs::create_dir_all(path)?
    }
    Ok(())
}

fn init_check() {
    if !Path::new(STATIC_RESOURCE_PATH).exists() {
        panic!("The static resource folder does not exist.");
    }
}

fn main() {
    init().expect("Initialization failed");
    init_check();
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80);
    let listener = TcpListener::bind(socket_addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let http_connect = HttpConnect { tcp: stream };
                http_connect.handle_connect();
            }
            Err(e) => {
                eprintln!("Connect Incoming Error:{}", e)
            }
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
