use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            if let Err(e) = handle_connection(stream) {
                eprintln!("error: {e}");
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap()?;

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("200 OK", "index.html")
    } else {
        ("404 NOT FOUND", "")
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response = format!(
        "HTTP/1.1 {status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;

    Ok(())
}
