use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_response(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_response(mut stream: TcpStream) {
    let response_data: &str = r"HTTP/1.1 200 OK\r\n\r\n";
    let mut buffer = [0; 2048];

    match stream.read(&mut buffer[..]) {
        Ok(bytes_no) => {
            println!("Readed {bytes_no} bytes");

            let data_rec: String = String::from_utf8(buffer.to_vec()).unwrap();
            println!("Received from client: {data_rec}");
        }
        Err(e) => println!("ERROR reading. Error: {e}"),
    }

    match stream.write(response_data.as_bytes()) {
        Ok(n) => println!("{n} bytes writed"),
        Err(_) => println!("Error writing bytes"),
    }
 
}
