pub(crate) enum WordToNumberError {
    BadRequest,
    InternalServer,
}

use crate::word_to_number::WordToNumberError::{BadRequest, InternalServer};

pub(crate) fn change_word_to_number(word_number: &str) -> Result<u64, WordToNumberError> {
    let mut number: u64 = 0;
    let word_number = word_number.to_lowercase();
    let mut word_numbers_mapped: Vec<Option<u64>> = Vec::new();
    let mut multiplier_indexes: Vec<usize> = Vec::new();
    if word_number.contains(" and ") {
        let parts = word_number.split(" and ").collect::<Vec<&str>>();
        word_numbers_mapped = parts
            .get(0)
            .unwrap()
            .split_whitespace()
            .map(|number| exchange_word_for_number(number))
            .collect::<Vec<Option<u64>>>();
        let numbers_after_and = match parts.get(1) {
            Some(smth) => smth,
            None => "",
        };
        if numbers_after_and.contains('-') {
            let numbers = numbers_after_and.split('-').collect::<Vec<&str>>();
            for number in numbers {
                word_numbers_mapped.push(exchange_word_for_number(number));
            }
        } else {
            word_numbers_mapped.push(exchange_word_for_number(numbers_after_and))
        }
    } else if word_number.contains('-') {
        let numbers = word_number.split('-').collect::<Vec<&str>>();
        for number in numbers {
            word_numbers_mapped.push(exchange_word_for_number(number));
        }
    } else {
        word_numbers_mapped = word_number
            .split_whitespace()
            .map(|number| exchange_word_for_number(number))
            .collect::<Vec<Option<u64>>>();
    }
    for (index, number) in word_numbers_mapped.iter().enumerate() {
        let number = match number {
            Some(number) => *number,
            None => return Err(BadRequest),
        };
        if is_multiplier_number(number) {
            multiplier_indexes.push(index)
        }
    }

    let mut elements_removed: usize = 0;
    let mut numbers_to_add = Vec::new();
    for mut index in multiplier_indexes {
        index -= elements_removed;
        let mut temp_number = 1;
        for i in 0..=index {
            temp_number *= word_numbers_mapped[i].unwrap(); // This is safe cuz the whole vector is checked above
        }
        numbers_to_add.push(temp_number);
        elements_removed += index + 1; // Added one since the elements removed != index since index starts at 0.
        word_numbers_mapped.drain(0..=index);
    }
    if !word_numbers_mapped.is_empty() {
        for num in word_numbers_mapped {
            let num = match num {
                Some(num) => num,
                None => return Err(BadRequest),
            };
            number += num
        }
    }
    for num in numbers_to_add {
        number += num
    }
    Ok(number)
}
fn exchange_word_for_number(number_word: &str) -> Option<u64> {
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
        "thousand" => Some(1000),
        _ => None,
    }
}

fn is_multiplier_number(number: u64) -> bool {
    number.to_string().ends_with("00")
}
