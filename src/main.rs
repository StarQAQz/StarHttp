/*
 * @Description: 程序入口
 * @Author: zhengzetao
 * @Date: 2022-07-05 13:21:18
 */
use std::{
    io::{self, Read},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};

fn main() -> Result<(), io::Error> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80);
    let listener = TcpListener::bind(socket_addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let line_one = read_line(&stream);
                let line_two = read_line(&stream);
                println!("{}", line_one?);
                println!("{}", line_two?);
            }
            Err(e) => {
                eprintln!("Connect Incoming Error:{}", e)
            }
        }
    }
    Ok(())
}

// fn handle_connect(stream: TcpStream) {
//     stream.read()
//     println!("Connect Incoming!");
// }

fn read_line(stream: &TcpStream) -> Result<String, io::Error> {
    let mut line = String::new();
    for buf in stream.bytes() {
        let val = buf? as char;
        if val == '\r' {
            if let Some(val) = stream.bytes().next() {
                if val? as char == '\n' {
                    return Ok(line);
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "read line error: Read only '\\r', no '\\n'",
                    ));
                }
            }
        } else {
            line.push(val);
        }
    }
    return Err(io::Error::new(
        io::ErrorKind::Other,
        "read line error: No newline character was read",
    ));
}
