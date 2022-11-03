use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    path::PathBuf,
};

use crate::{config::MyConfig, error::HttpError, hex, log_error, log_info};

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
    fn get_status_default_html(&self) -> &str {
        match self {
            HttpStatus::OK => "",
            HttpStatus::NotFound => "<!DOCTYPE html><head><title>404 NOT FOUND</title></head><body><h1>404 NOT FOUND!</h1></body></html>",
            HttpStatus::InternalServerError => "<!DOCTYPE html><head><title>500 INTERNAL SERVER ERROR</title></head><body><h1>500 INTERNAL SERVER ERROR!</h1></body></html>",
        }
    }
}

struct ResponseHeader<'a> {
    http_status: &'a HttpStatus,
    params: HashMap<&'a str, String>,
}

impl ResponseHeader<'_> {
    fn get(self) -> String {
        let mut header = String::new();
        header.push_str(self.http_status.get_http_status());
        for (key, val) in self.params.iter() {
            let param = format!("{}:{}\r\n", key, val);
            header.push_str(&param);
        }
        header.push_str("\r\n");
        return header;
    }
}

struct RequestHeader {
    params: HashMap<String, String>,
}

impl RequestHeader {
    //读取请求标头
    fn read_request_header(stream: &TcpStream) -> Result<RequestHeader, HttpError> {
        let mut params: HashMap<String, String> = HashMap::new();
        while let Some(line) = read_line(&stream)? {
            let kv: Vec<&str> = line.split(':').map(|h| h.trim()).collect();
            params.insert(
                kv.get(0).unwrap().to_lowercase(),
                kv.get(1).unwrap().to_string(),
            );
        }
        Ok(RequestHeader { params })
    }

    fn get_first_accept(&self) -> Option<String> {
        if let Some(accept) = self.params.get("accept") {
            let accepts: Vec<&str> = accept.split(',').map(|a| a.trim()).collect();
            if let Some(accept) = accepts.get(0) {
                return Option::Some((*accept).to_owned());
            }
        }
        Option::None
    }
}

trait ResponseBody {
    fn write_in_connect(&self, stream: &TcpStream) -> Result<(), HttpError>;
}

impl ResponseBody for File {
    fn write_in_connect(&self, stream: &TcpStream) -> Result<(), HttpError> {
        let mut tcp = stream;
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
    fn write_in_connect(&self, stream: &TcpStream) -> Result<(), HttpError> {
        let mut tcp = stream;
        tcp.write_all(self.as_bytes())?;
        tcp.flush()?;
        Ok(())
    }
}

//连接控制，读取请求并判断请求类型
pub fn handle_connect(stream: TcpStream) {
    if let Ok(first_line) = read_line(&stream) {
        if let Some(first_line) = first_line {
            //读取请求第一行参数
            log_info!("{}", first_line);
            let mut header = first_line.split_whitespace();
            let request_type = header.next().unwrap();
            let url = hex::url_decoding(header.next().unwrap().to_string());
            //读取请求头
            match RequestHeader::read_request_header(&stream) {
                Ok(request_header) => {
                    //分发请求类型处理
                    match request_type.to_lowercase().as_str() {
                        "get" => {
                            if let Err(e) = get(&stream, request_header, url) {
                                log_error!("The GET request is abnormal. Error reason: {}", e);
                                if let Err(e) =
                                    send_failed(&stream, &HttpStatus::InternalServerError)
                                {
                                    log_error!("Response 500 failed. Error reason: {}", e);
                                }
                            }
                        }
                        val => {
                            log_error!("Do not support request type! Request type: {}", val);
                        }
                    };
                }
                Err(e) => log_error!("The read request header is abnormal! Err:{}", e),
            }
        }
    }
    shutdown(stream);
}

//GET请求
fn get(stream: &TcpStream, request_header: RequestHeader, url: String) -> Result<(), HttpError> {
    //解析url，分隔参数
    let mut path = url.as_str();
    if url.contains("?") {
        let v: Vec<&str> = url.split("?").collect();
        path = v[0];
    }
    //构建文件路径
    let mut current_path = PathBuf::from(MyConfig::new().static_resource_path);
    if current_path.is_absolute() {
        current_path = current_path.canonicalize()?;
    }
    for node in path.split("/") {
        current_path = current_path.join(node);
    }
    if current_path.exists() && current_path.is_file() {
        match File::open(current_path.as_path()) {
            Ok(file) => {
                send_ok(stream, request_header, file)?;
                log_info!("GET {} SUCCESS!", url);
            }
            Err(e) => return Result::Err(HttpError::from(e)),
        }
    } else {
        send_failed(stream, &HttpStatus::NotFound)?;
    }
    Ok(())
}

/*
 * 读取一行数据
 */
fn read_line(stream: &TcpStream) -> Result<Option<String>, HttpError> {
    let mut line = String::new();
    let tcp = stream;
    for buf in tcp.bytes() {
        let val = buf? as char;
        if val == '\r' {
            let val = tcp.bytes().next().unwrap();
            if val? as char != '\n' {
                return Err(HttpError {
                    kind: "read_line".to_string(),
                    message: "read line error: Read only '\\r', no '\\n'".to_string(),
                });
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

fn send_ok(stream: &TcpStream, request_header: RequestHeader, file: File) -> Result<(), HttpError> {
    let mut params: HashMap<&str, String> = HashMap::new();
    match request_header.get_first_accept() {
        Some(accept) => params.insert("Content-Type", format!("{}; charset=utf-8", accept)),
        _ => params.insert("Content-Type", String::from("text/html; charset=utf-8")),
    };
    if let Ok(metadata) = file.metadata() {
        params.insert("Content-Length", metadata.len().to_string());
    }
    let header = ResponseHeader {
        http_status: &HttpStatus::OK,
        params,
    };
    send(stream, header, Some(&file))
}

fn send_failed(stream: &TcpStream, http_status: &HttpStatus) -> Result<(), HttpError> {
    let html = http_status.get_status_default_html();
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert(
        "Content-Type",
        String::from("text/html;text/html; charset=utf-8"),
    );
    params.insert("Content-Length", html.len().to_string());
    let header = ResponseHeader {
        http_status,
        params,
    };
    send(stream, header, Some(&html.to_string()))
}

fn send(
    stream: &TcpStream,
    response_header: ResponseHeader,
    body: Option<&dyn ResponseBody>,
) -> Result<(), HttpError> {
    let mut tcp = stream;
    tcp.write_all(response_header.get().as_bytes())?;
    if let Some(body) = body {
        body.write_in_connect(stream)?;
    }
    Ok(())
}

fn shutdown(stream: TcpStream) {
    if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
        log_error!(
            "Failed to shutdown the connection. Error reason: {}",
            e.to_string()
        );
    }
}
