/*
 * @Description: 程序入口
 * @Author: zhengzetao
 * @Date: 2022-07-05 13:21:18
 */
use std::{
    collections::HashMap,
    fmt::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
};

const STATIC_RESOURCE_PATH: &str = "./static";
const LOG_PATH: &str = "./log";
const POOL_SIZE: usize = 10;

//连接控制，读取请求并判断请求类型
fn handle_connect(stream: TcpStream) {
    if let Ok(first_line) = read_line(&stream) {
        if let Some(first_line) = first_line {
            println!("{}", &first_line);
            let mut header = first_line.split_whitespace();
            let request_type = header.next().unwrap();
            let url = header.next().unwrap();
            //分发请求类型处理
            match request_type.to_lowercase().as_str() {
                "get" => {
                    if let Err(e) = get(&stream, url) {
                        eprintln!(
                            "The GET request is abnormal. Error reason: {}",
                            e.to_string()
                        );
                        if let Err(e) = send_internal_server_error(&stream) {
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
    shutdown(stream);
}

//GET请求
fn get(stream: &TcpStream, url: &str) -> io::Result<()> {
    //读取剩余请求
    while let Some(_) = read_line(&stream)? {}
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
        Ok(file) => send_ok(stream, file)?,
        Err(_) => send_not_found(stream)?,
    }
    Ok(())
}

fn send_ok(stream: &TcpStream, file: File) -> io::Result<()> {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("Content-Type", "text/html;text/html; charset=utf-8\r\n");
    let header = ResponseHeader {
        http_status: HttpStatus::OK,
        params,
    };
    send(stream, header, Some(&file))
}

fn send_not_found(stream: &TcpStream) -> io::Result<()> {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("Content-Type", "text/html;text/html; charset=utf-8\r\n");
    let header = ResponseHeader {
        http_status: HttpStatus::NotFound,
        params,
    };
    let html = "<!DOCTYPE html><head><title>404 NOT FOUND</title></head><body><h1>404 NOT FOUND!</h1></body></html>";
    send(stream, header, Some(&html.to_string()))
}

fn send_internal_server_error(stream: &TcpStream) -> io::Result<()> {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("Content-Type", "text/html;text/html; charset=utf-8\r\n");
    let header = ResponseHeader {
        http_status: HttpStatus::InternalServerError,
        params,
    };
    let html = "<!DOCTYPE html><head><title>500 INTERNAL SERVER ERROR</title></head><body><h1>500 INTERNAL SERVER ERROR!</h1></body></html>";
    send(stream, header, Some(&html.to_string()))
}

fn send(
    stream: &TcpStream,
    header: ResponseHeader,
    body: Option<&dyn ResponseBody>,
) -> io::Result<()> {
    let mut tcp = stream;
    tcp.write_all(header.get().as_bytes())?;
    if let Some(body) = body {
        body.write_in_connect(stream)?;
    }
    Ok(())
}

fn shutdown(stream: TcpStream) {
    if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
        eprintln!(
            "Failed to shutdown the connection. Error reason: {}",
            e.to_string()
        );
    }
}

/*
 * 读取一行数据
 */
fn read_line(stream: &TcpStream) -> io::Result<Option<String>> {
    let mut line = String::new();
    let tcp = stream;
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
    fn write_in_connect(&self, stream: &TcpStream) -> io::Result<()>;
}

impl ResponseBody for File {
    fn write_in_connect(&self, stream: &TcpStream) -> io::Result<()> {
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
    fn write_in_connect(&self, stream: &TcpStream) -> io::Result<()> {
        let mut tcp = stream;
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

struct Worker {
    id: usize,
    work: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let work = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job()
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            work: Option::Some(work),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> Result<ThreadPool, Error> {
        if size <= 0 {
            eprintln!("The thread pool size must be greater than 0")
        }
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        Ok(ThreadPool { workers, sender })
    }

    fn exec<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(work) = worker.work.take() {
                work.join().unwrap();
            }
        }
    }
}

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
