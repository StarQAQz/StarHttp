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

const STATIC_SOURCE_PATH: &str = "./static";

struct HttpConnect {
    tcp: TcpStream,
}

impl HttpConnect {
    //连接控制，读取请求并判断请求类型
    fn handle_connect(self) -> io::Result<()> {
        if let Some(first_line) = self.read_line()? {
            println!("{}", &first_line);
            let mut header = first_line.split_whitespace();
            let request_type = header.next().unwrap();
            let url = header.next().unwrap();
            //分发请求类型处理
            match request_type.to_lowercase().as_str() {
                "get" => self.get(url)?,
                val => {
                    eprintln!("Do not support request type! Request type: {}", val);
                    return Ok(());
                }
            };
        }
        Ok(())
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
        let mut current_path = fs::canonicalize(Path::new(STATIC_SOURCE_PATH))?;
        for node in path.split("/") {
            current_path.push(node);
        }
        println!("{}", current_path.display().to_string());
        if current_path.is_dir() {}
        let file = File::open(current_path)?;
        self.send_ok(file)?;
        Ok(())
    }

    fn send_ok(&self, file: File) -> io::Result<()> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("Content-Type", "text/html;text/html; charset=utf-8\r\n");
        let header = ResponseHeader {
            http_status: HttpStatus::OK,
            params,
        };
        self.send(header, file)
    }

    fn send(&self, header: ResponseHeader, file: File) -> io::Result<()> {
        let mut tcp = &self.tcp;
        tcp.write_all(header.get().as_bytes())?;
        tcp.write_all(b"\r\n")?;
        let mut buf_reader = BufReader::new(file);
        while buf_reader.fill_buf()?.len() > 0 {
            let size = tcp.write(buf_reader.buffer())?;
            buf_reader.consume(size);
        }
        tcp.flush()?;
        tcp.shutdown(std::net::Shutdown::Both)?;
        Ok(())
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
        header.push_str(self.http_status.getHttpStatus());
        for (key, val) in self.params.iter() {
            let param = format!("{}:{}\r\n", key, val);
            header.push_str(&param);
        }
        return header;
    }
}

enum HttpStatus {
    OK,                    //"HTTP/1.0 200 OK\r\n"
    NOT_FOUND,             //"HTTP/1.0 400 NOT FOUND\r\n"
    INTERNAL_SERVER_ERROR, //"HTTP/1.0 500 INTERNAL SERVER ERROR\r\n"
}

impl HttpStatus {
    fn getHttpStatus(&self) -> &str {
        match self {
            OK => "HTTP/1.0 200 OK\r\n",
            NOT_FOUND => "HTTP/1.0 404 NOT FOUND\r\n",
            INTERNAL_SERVER_ERROR => "HTTP/1.0 500 INTERNAL SERVER ERROR\r\n",
        }
    }
}

fn main() -> Result<(), io::Error> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80);
    let listener = TcpListener::bind(socket_addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let http_connect = HttpConnect { tcp: stream };
                http_connect.handle_connect()?
            }
            Err(e) => {
                eprintln!("Connect Incoming Error:{}", e)
            }
        }
    }
    Ok(())
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
