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
    let mut response_data: String = String::from("HTTP/1.1 200 OK \r\n\r\n");
    let mut buffer: [u8; 1024] = [0; 1024];

    match stream.read(&mut buffer[..]) {
        Ok(bytes_no) => {
            println!("Readed {bytes_no} bytes");

            let data_rec: String = String::from_utf8(buffer.to_vec()).unwrap();
            let parsed_request = parse_request(data_rec);

            if parsed_request.path == "/" {
                response_data = "HTTP/1.1 200 OK \r\n\r\n".into()
            } 

            else if parsed_request.path.starts_with("/echo") {

               println!("{}",parsed_request.path.split_at(6).1);
                let body:String  = parsed_request.path.split_at(6).1.into();
                let mut response = Response::default();
                response.header_1 = "HTTP/1.1 200 OK \r\n".into();
                response.content_type = "text-plain\r\n".into();
                response.content_lenght = body.len().to_string();
                response.content_lenght.push_str("\r\n");
                response.two_space = "\r\n\r\n".into();
                response.body = body;

                response_data =format!("{}{}{}{}{}",response.header_1,response.content_type,response.content_lenght,response.two_space,response.body);
                println!("{}",response_data);
            }
            
            else {
                response_data = "HTTP/1.1 404 NOT FOUND \r\n\r\n".into();
            }
        }
        Err(e) => println!("ERROR reading. Error: {e}"),
    }

    match stream.write(response_data.as_bytes()) {
        Ok(n) => println!("{n} bytes writed"),
        Err(_) => println!("Error writing bytes"),
    }
}

fn parse_request(received: String) -> RequestData {
    let lines: Vec<_> = received.lines().collect();
    let mut parsed_data = RequestData::default();
    let splited: Vec<_> = lines[0].split_whitespace().collect();
    parsed_data.method = splited[0].into();
    parsed_data.path = splited[1].into();
    parsed_data.http_version = splited[2].into();

    parsed_data
}
#[derive(Default, Debug)]
struct RequestData {
    method: String,
    path: String,
    http_version: String,
}
#[derive(Default, Debug)]
struct Response{
    header_1 :String,
    content_type: String,
    content_lenght: String,
    two_space: String,
    body: String
}

fn parse_response(data: &str){}