pub (crate) enum WordToNumberError {
    BadRequest,
    InternalServer
}

use crate::word_to_number::WordToNumberError::{BadRequest, InternalServer};

pub (crate) fn change_word_to_number(word_number: &str) -> Result<u16, WordToNumberError> {

    let number: u16;

    if word_number.contains("and") {
        let numbers = word_number.split("and");
        let mut temp_number = 1;
        for mut number in numbers {
            if number.starts_with(' ') {
                number = match number.strip_prefix(' ') {
                    Some(number) => number,
                    None => return Err(InternalServer)
                }
            }
            if number.contains(' ') {
                let numbers = number.split_ascii_whitespace();
                for number in numbers {
                    if !number.eq(" ") {
                        temp_number *= match exchange_word_for_number(number) {
                            Some(num) => num,
                            None => return Err(BadRequest)
                        };
                    }
                }
            }
            if number.contains('-') {
                let numbers = number.split("-");
                for number in numbers {
                    temp_number += match exchange_word_for_number(number) {
                            Some(num) => num,
                            None => return Err(BadRequest)
                        };
                }
            }
        }
        number = temp_number;
    }
    else if word_number.contains('-') {
        let numbers = word_number.split('-');
        let mut temp_number = 0;
        for number in numbers {
            temp_number += match exchange_word_for_number(number) {
                            Some(num) => num,
                            None => return Err(BadRequest)
                        };
        }
        number = temp_number;
    }
    else if word_number.contains("hundred") {
        let mut temp_number = 1;
        if word_number.contains(' ') {
            let numbers = word_number.split_ascii_whitespace();
            for number in numbers {
                if !number.eq(" ") {
                    temp_number *= match exchange_word_for_number(number) {
                            Some(num) => num,
                            None => return Err(BadRequest)
                        };
                }
            }
        }
        number = temp_number
    }
    else {
        number = match exchange_word_for_number(&*word_number) {
            Some(num) => num,
            None => return Err(BadRequest)
        };
    }

    println!("Received: {word_number}; Returned: {number}");
    Ok(number)
}
fn exchange_word_for_number(number_word: &str) -> Option<u16> {
    match number_word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        "eleven" => Some(11),
        "twelve" => Some(12),
        "thirteen" => Some(13),
        "fourteen" => Some(14),
        "fifteen" => Some(15),
        "sixteen" => Some(16),
        "seventeen" => Some(17),
        "eighteen" => Some(18),
        "nineteen" => Some(19),
        "twenty" => Some(20),
        "thirty" => Some(30),
        "forty" => Some(40),
        "fifty" => Some(50),
        "sixty" => Some(60),
        "seventy" => Some(70),
        "eighty" => Some(80),
        "ninety" => Some(90),
        "hundred" => Some(100),
        _ => None
    }
}