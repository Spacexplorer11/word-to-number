mod word_to_number;

use crate::word_to_number::{WordToNumberError, change_word_to_number};
use std::time::Duration;
use std::{
    io::{BufReader, ErrorKind, prelude::*},
    net::{TcpListener, TcpStream},
    str::from_utf8,
};

enum StatusCodes {
    Ok,
    BadRequest,
    Timeout,
    InternalServer,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            _ => continue,
        };

        TcpStream::set_read_timeout(&stream, Some(Duration::from_secs(5))).expect(
            "Unable to set read timeout for stream. Panicking to avoid resource exploitation",
        );
        TcpStream::set_write_timeout(&stream, Some(Duration::from_secs(5))).expect(
            "Unable to set write timeout for stream. Panicking to avoid resource exploitation",
        );

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let buf_reader_bytes = buf_reader.bytes();

    let mut http_request = String::new();
    let mut http_body = String::new();

    let mut crlf: u8 = 0; // Carriage Return Line Feed (new stuff I learned during this project)
    let mut body_section: bool = false;

    for byte in buf_reader_bytes {
        if let Err(e) = byte {
            eprintln!("Whoopsies... {e}");
            match e.kind() {
                ErrorKind::WouldBlock | ErrorKind::TimedOut => {
                    send_response(&stream, StatusCodes::Timeout, None)
                }
                _ => send_response(&stream, StatusCodes::InternalServer, None),
            }
            return;
        }
        let byte = byte.expect("If this causes a crash, something really messed up");
        if byte.eq(&13) || byte.eq(&10) {
            crlf += 1;
            http_request.push_str(from_utf8(&[byte]).unwrap());
        } else if crlf == 4 {
            body_section = true;
            crlf = 0;
        } else if crlf < 3 && !body_section {
            crlf = 0;
            http_request.push_str(from_utf8(&[byte]).unwrap());
            if http_request.contains("Content-Length: 0") {
                send_response(&stream, StatusCodes::BadRequest, None);
                return;
            }
        } else if body_section && !byte.eq(&125) {
            http_body.push_str(from_utf8(&[byte]).unwrap());
        } else {
            break;
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("Request: {http_request}");
        println!("Body: {http_body}");
    }

    let mut numbers_from_words: Vec<u16> = Vec::new();

    for line in http_body.split(',') {
        if line.contains("word") {
            let parts = line.split(':').collect::<Vec<_>>();
            let word_with_quotes = match parts.get(1) {
                Some(word) => word,
                None => {
                    send_response(&stream, StatusCodes::BadRequest, None);
                    return;
                }
            };
            let parts = word_with_quotes.split("\"").collect::<Vec<_>>();
            let word = match parts.get(1) {
                Some(word) => word,
                None => {
                    send_response(&stream, StatusCodes::BadRequest, None);
                    return;
                }
            };
            numbers_from_words.push(match change_word_to_number(word) {
                Ok(num) => num,
                Err(WordToNumberError::BadRequest) => {
                    send_response(&stream, StatusCodes::BadRequest, None);
                    return;
                }
                Err(WordToNumberError::InternalServer) => {
                    send_response(&stream, StatusCodes::InternalServer, None);
                    return;
                }
            });
        }
    }
    send_response(&stream, StatusCodes::Ok, Some(numbers_from_words))
}

fn send_response(
    mut stream: &TcpStream,
    status_code: StatusCodes,
    numbers_from_words: Option<Vec<u16>>,
) {
    let status_line = match status_code {
        StatusCodes::Ok => "HTTP/1.1 200 OK \r\n",
        StatusCodes::BadRequest => "HTTP/1.1 400 BAD REQUEST \r\n",
        StatusCodes::Timeout => "HTTP/1.1 408 REQUEST TIMEOUT\r\n",
        StatusCodes::InternalServer => "HTTP/1.1 500 INTERNAL SERVER ERROR \r\n",
    };
    let default_headers = "Connection: close\r\n\r\n";
    let ok_headers = "Cache-Control: public, max-age=604800, s-maxage=604800, immutable\r\n";

    let response = match status_code {
        StatusCodes::Ok => {
            let numbers_from_words = match numbers_from_words {
                Some(num) => num,
                None => return,
            };
            if numbers_from_words.len() > 0 {
                let mut returned_json = format!("\"number\": {}", numbers_from_words[0]);
                if numbers_from_words.len() > 1 {
                    let mut numbers_from_words_iter = numbers_from_words.iter();
                    numbers_from_words_iter.next();
                    let mut i: u128 = 0;
                    for number in numbers_from_words_iter {
                        i += 1;
                        returned_json.push_str(&*format!(",\n\"number-{i}\": {number}"))
                    }
                }
                let content_length = format!("{{{returned_json}}}")
                    .as_bytes()
                    .into_iter()
                    .count();
                format!(
                    "{status_line}Content-Type: application/json\r\nContent-Length: {content_length}\r\n{ok_headers}{default_headers}{{{returned_json}}}"
                )
            } else {
                send_response(stream, StatusCodes::BadRequest, None);
                return;
            }
        }
        _ => {
            let content_length = "{\"error\": \"You received an error. Please check my README at https://github.com/spacexplorer11/word-to-number/blob/main/README.md for more details.\"}".len();
            format!(
                "{status_line}Content-Type: application/json\r\nContent-Length: {content_length}\r\n{default_headers}{{\"error\": \"You received an error. Please check my README at https://github.com/spacexplorer11/word-to-number/blob/main/README.md for more details.\"}}"
            )
        }
    };

    #[cfg(debug_assertions)]
    println!("Response: {response}");

    if let Err(err) = stream.write_all(response.as_bytes()) {
        #[cfg(debug_assertions)]
        eprintln!("Failed to write response to client: {err}");
        return;
    }
    if let Err(err) = stream.flush() {
        #[cfg(debug_assertions)]
        eprintln!("Failed to flush response to client: {err}");
        return;
    }
}
