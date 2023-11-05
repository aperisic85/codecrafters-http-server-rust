use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const USERAGENT: &str = "/user-agent";
const ECHO: &str = "/echo";
fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");

                thread::spawn(|| {
                    handle_response(stream);
                });
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
            } else if parsed_request.path.starts_with(ECHO) {
                println!("{}", parsed_request.path.split_at(ECHO.len()).1);
                let body: &str = parsed_request.path.split_at(6).1;

                let response = parse_response(body);
                response_data = format!(
                    "{}{}{}{}{}",
                    response.header_1,
                    response.content_type,
                    response.content_lenght,
                    response.two_space,
                    response.body
                );
                println!("{}", response_data);
            } else if parsed_request.path.starts_with(USERAGENT) {
                //let body: &str = parsed_request.path.split_at(parsed_request.path.len()).1;
                println!("{}", parsed_request.user_agent);
                let response = parse_response_agent(&parsed_request);
                response_data = format!(
                    "{}{}{}{}{}",
                    response.header_1,
                    response.content_type,
                    response.content_lenght,
                    response.two_space,
                    response.body,
                );
                println!("PRINT RESPONSE::::::{}", response_data);
            } else {
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

#[derive(Default, Debug)]
struct RequestData {
    method: String,
    path: String,
    http_version: String,
    host: String,
    user_agent: String,
}
#[derive(Default, Debug)]
struct Response<'a> {
    header_1: String,
    content_type: String,
    content_lenght: String,

    two_space: String,
    body: &'a str,
}

fn parse_request(received: String) -> RequestData {
    let lines: Vec<_> = received.lines().collect();
    let mut parsed_data = RequestData::default();
    let splited: Vec<_> = lines[0].split_whitespace().collect();
    let host: Vec<_> = lines[1].split(": ").collect();
    let user_agent: Vec<_> = lines[2].split(": ").collect();
    parsed_data.method = splited[0].into();
    parsed_data.path = splited[1].into();
    parsed_data.http_version = splited[2].into();
    parsed_data.host = host[1].into();
    parsed_data.user_agent = user_agent[1].into();

    parsed_data
}

fn parse_response(data: &str) -> Response {
    let mut response = Response::default();
    response.header_1 = "HTTP/1.1 200 OK\r\n".into();
    response.content_type = "Content-Type: text/plain\r\n".into();
    response.content_lenght = "Content-Length:".into();
    response.content_lenght.push_str(&data.len().to_string());
    response.content_lenght.push_str("\r\n");
    response.two_space = "\r\n".into();
    response.body = data.into();

    response
}

fn parse_response_agent(data: &RequestData) -> Response {
    let mut response = Response::default();
    response.header_1 = "HTTP/1.1 200 OK\r\n".into();
    response.content_type = "Content-Type: text/plain\r\n".into();
    response.content_lenght = "Content-Length:".into();
    response
        .content_lenght
        .push_str(&data.user_agent.len().to_string());
    response.content_lenght.push_str("\r\n");
    response.two_space = "\r\n".into();
    response.body = &data.user_agent;

    response
}
