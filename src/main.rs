mod word_to_number;

use std::{
  io::{BufReader, prelude::*},
  net::{TcpListener, TcpStream}
};
use crate::word_to_number::{change_word_to_number, WordToNumberError};

fn main() {
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let buf_reader_iter = buf_reader
        .lines()
        .map(|result| result.unwrap());

    let mut http_request: Vec<_> = Vec::new();
    let mut http_body: Vec<_> = Vec::new();

    let mut body_section: bool = false;
    let mut body_bytes_read: usize = 0;
    let mut total_body_bytes: usize = 0;

    for line in buf_reader_iter {
        if !line.is_empty() && !body_section {
            if line.contains("Content-Length") {
                total_body_bytes = line.split(':').collect::<Vec<_>>()[1].trim().parse().unwrap();
                // println!("Total body bytes = {total_body_bytes}");
            }
            http_request.push(line)
        }
        else if !body_section {
            body_section = true;
        }
        else if body_section {
            body_bytes_read += line.as_bytes().iter().count() + "\r\n".as_bytes().iter().count(); // this is stripped out earlier by .lines() so we have to add it to the count otherwise it never reaches the total
            // println!("Total body bytes read = {body_bytes_read}");
            http_body.push(line);
            if body_bytes_read >= total_body_bytes { break; }
        }
    }

    // println!("Request: {http_request:#?}");
    // println!("Body: {http_body:#?}");

    let mut number_from_word: u16 = 0;

    for line in http_body {
        if line.contains("word") {
            let word_with_quotes: &str = line.split(':').collect::<Vec<_>>()[1];
            let word = word_with_quotes.split("\"").collect::<Vec<_>>()[1];
            number_from_word = match change_word_to_number(word) {
                Ok(num) => num,
                Err(WordToNumberError::BadRequest) => {stream.write_all("HTTP/1.1 400 BAD REQUEST \r\n\r\n".as_bytes()).unwrap(); return},
                Err(WordToNumberError::InternalServer) => {stream.write_all("HTTP/1.1 500 INTERNAL SERVER ERROR \r\n\r\n".as_bytes()).unwrap(); return}
            };
            break;
        }
    }

    let status_line = "HTTP/1.1 200 OK \r\n";
    let returned_json = format!("{{\"number\": {}}}", number_from_word);
    let content_length = returned_json.as_bytes().into_iter().count();

    let response = format!(
        "{status_line}Content-Type: application/json\r\nContent-Length: {content_length}\r\n\r\n{returned_json}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}