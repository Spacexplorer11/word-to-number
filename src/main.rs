mod word_to_number;

use std::{
  io::{BufReader, prelude::*},
  net::{TcpListener, TcpStream}
};
use crate::word_to_number::{change_word_to_number, WordToNumberError};

enum StatusCodes {
   Ok,
    BadRequest,
    InternalServer
}

fn main() {
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
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
                Err(WordToNumberError::BadRequest) => {send_response(stream, StatusCodes::BadRequest, None); return},
                Err(WordToNumberError::InternalServer) => {send_response(stream, StatusCodes::InternalServer, None); return}
            };
            break;
        }
    }

    send_response(stream, StatusCodes::Ok, Some(number_from_word))
}

fn send_response(mut stream: TcpStream, status_code: StatusCodes, number_from_word: Option<u16>) {
    let status_line = match status_code {
        StatusCodes::Ok => "HTTP/1.1 200 OK \r\n",
        StatusCodes::BadRequest => "HTTP/1.1 400 BAD REQUEST \r\n",
        StatusCodes::InternalServer => "HTTP/1.1 500 INTERNAL SERVER ERROR \r\n"
    };
    let default_headers = "Connection: close\r\nCache-Control: public, max-age=604800, s-maxage=604800, immutable\r\n\r\n";

    let response = match status_code {
        StatusCodes::Ok => {
            let number_from_word = match number_from_word {
                Some(num) => num,
                None => return
            };
            let returned_json = format!("{{\"number\": {number_from_word}}}");
            let content_length = returned_json.as_bytes().into_iter().count();
            format!("{status_line}Content-Type: application/json\r\nContent-Length: {content_length}\r\n{default_headers}{returned_json}")
        }
        _ => format!("{status_line}{default_headers}")
    };

    // println!("Response: {response}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}