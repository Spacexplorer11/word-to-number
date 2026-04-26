mod word_to_number;

use crate::word_to_number::{WordToNumberError, change_word_to_number};
use std::time::Duration;
use std::{
    io::{BufReader, ErrorKind, prelude::*},
    net::{TcpListener, TcpStream},
};
use time::UtcDateTime;

enum StatusCodes {
    Ok,
    BadRequest,
    Timeout,
    LengthRequired,
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
    let http_body: String;

    {
        let buf_reader = BufReader::new(&stream);
        let buf_reader_bytes = buf_reader.bytes();
        let mut http_headers_bytes: Vec<u8> = Vec::new();
        let mut http_headers = String::new();
        let mut http_body_bytes: Vec<u8> = Vec::new();
        let mut crlf: u8 = 0; // Carriage Return Line Feed (new stuff I learned during this project)
        let mut body_section: bool = false;
        let mut headers;
        let mut content_length = 0;
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
            if (byte.eq(&13) || byte.eq(&10)) && !body_section {
                crlf += 1;
                http_headers_bytes.push(byte);
                if crlf >= 4 {
                    body_section = true;
                    crlf = 0;
                    http_headers =
                        match String::from_utf8_lossy(&*http_headers_bytes).parse::<String>() {
                            Ok(string) => string.to_lowercase(),
                            _ => {
                                send_response(&stream, StatusCodes::BadRequest, None);
                                return;
                            }
                        };
                    if !http_headers.contains("content-length:") {
                        send_response(&stream, StatusCodes::LengthRequired, None);
                        return;
                    }
                    headers = http_headers.split("\r\n").collect::<Vec<_>>();
                    for header in headers {
                        if header.contains("content-length:") {
                            let content_length_part =
                                header.split("content-length:").collect::<Vec<_>>();
                            match content_length_part.get(1) {
                                Some(content_length_str) => {
                                    match content_length_str.trim().parse::<usize>() {
                                        Ok(num) => {
                                            if num == 0 {
                                                send_response(
                                                    &stream,
                                                    StatusCodes::BadRequest,
                                                    None,
                                                );
                                                return;
                                            } else {
                                                content_length = num;
                                            }
                                        }
                                        _ => {
                                            send_response(&stream, StatusCodes::BadRequest, None);
                                            return;
                                        }
                                    }
                                }
                                _ => {
                                    send_response(&stream, StatusCodes::LengthRequired, None);
                                    return;
                                }
                            }
                        }
                    }
                }
            } else if crlf < 3 && !body_section {
                crlf = 0;
                http_headers_bytes.push(byte);
            } else if body_section {
                http_body_bytes.push(byte);
                content_length -= 1;
                if content_length == 0 {
                    break;
                }
            }
        }
        http_body = match String::from_utf8_lossy(&*http_body_bytes).parse() {
            Ok(string) => string,
            _ => {
                send_response(&stream, StatusCodes::BadRequest, None);
                return;
            }
        };
        #[cfg(debug_assertions)]
        {
            println!("Request: {http_headers}");
            println!("Body: {http_body}");
        }
    }

    let mut numbers_from_words: Vec<u16> = Vec::new();
    let mut words_received: Vec<&str> = Vec::new();

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
            match parts.get(1) {
                Some(word) => words_received.push(word),
                None => {
                    send_response(&stream, StatusCodes::BadRequest, None);
                    return;
                }
            };
        }
    }
    for word in &words_received {
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
    #[cfg(debug_assertions)]
    println!(
        "[{}] Received {words_received:?}; \nReturned {numbers_from_words:?};",
        UtcDateTime::now()
    );

    #[cfg(not(debug_assertions))]
    println!("[{}] Successfully processed request", UtcDateTime::now());

    send_response(&stream, StatusCodes::Ok, Some(numbers_from_words))
}

fn send_response(
    mut stream: &TcpStream,
    status_code: StatusCodes,
    numbers_from_words: Option<Vec<u16>>,
) {
    let status_line = match status_code {
        StatusCodes::Ok => "HTTP/1.1 200 OK\r\n",
        StatusCodes::BadRequest => "HTTP/1.1 400 Bad Request\r\n",
        StatusCodes::Timeout => "HTTP/1.1 408 Request Timeout\r\n",
        StatusCodes::LengthRequired => "HTTP/1.1 411 Length Required\r\n",
        StatusCodes::InternalServer => "HTTP/1.1 500 Internal Server Error\r\n",
    };
    let default_headers = "Connection: close\r\n\r\n";
    let ok_headers = "Cache-Control: public, max-age=604800, s-maxage=604800, immutable\r\n";

    let response = match status_code {
        StatusCodes::Ok => {
            let numbers_from_words = numbers_from_words.expect(
                "You messed up and didn't pass the number from words array but wanted to send OK",
            );
            if !numbers_from_words.is_empty() {
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
                returned_json = format!("{{{returned_json}}}");
                format!(
                    "{status_line}Content-Type: application/json\r\nContent-Length: {}\r\n{ok_headers}{default_headers}{returned_json}",
                    returned_json.len()
                )
            } else {
                send_response(stream, StatusCodes::BadRequest, None);
                return;
            }
        }
        StatusCodes::BadRequest => {
            let error_message = "{\"error\": \"400 Bad Request\", \"message\":\"You might have a typo? Additionally, check my README at https://github.com/spacexplorer11/word-to-number/blob/main/README.md for more details.\"}";
            format!(
                "{status_line}Content-Type: application/json\r\nContent-Length: {}\r\n{default_headers}{error_message}",
                error_message.len()
            )
        }
        StatusCodes::Timeout => {
            let error_message = "{\"error\": \"408 Timeout\", \"message\":\"Please check my README at https://github.com/spacexplorer11/word-to-number/blob/main/README.md for more details.\"}";
            format!(
                "{status_line}Content-Type: application/json\r\nContent-Length: {}\r\n{default_headers}{error_message}",
                error_message.len()
            )
        }
        StatusCodes::LengthRequired => {
            let error_message = "{\"error\": \"411 Length Required\", \"message\":\"Please provide a Content-Length header. Additionally, check my README at https://github.com/spacexplorer11/word-to-number/blob/main/README.md for more details.\"}";
            format!(
                "{status_line}Content-Type: application/json\r\nContent-Length: {}\r\n{default_headers}{error_message}",
                error_message.len()
            )
        }
        StatusCodes::InternalServer => {
            let error_message = "{\"error\": \"500 Internal Server Error\", \"message\":\"Please try again later. Additionally, check my README at https://github.com/spacexplorer11/word-to-number/blob/main/README.md for more details.\"}";
            format!(
                "{status_line}Content-Type: application/json\r\nContent-Length: {}\r\n{default_headers}{error_message}",
                error_message.len()
            )
        }
    };

    #[cfg(debug_assertions)]
    println!("Response: {response}");

    if let Err(err) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response to client: {err}");
        return;
    }
    if let Err(err) = stream.flush() {
        eprintln!("Failed to flush response to client: {err}");
        return;
    }
}
