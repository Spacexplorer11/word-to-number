pub(crate) enum WordToNumberError {
    BadRequest,
}

use crate::word_to_number::WordToNumberError::BadRequest;

pub(crate) fn change_word_to_number(word_number: &str) -> Result<u64, WordToNumberError> {
    let mut number: u64 = 0;
    let mut word_numbers_mapped = Vec::new();
    let word_number = word_number.to_lowercase();
    let mut word_numbers_mapped_option_possible: Vec<Option<u64>>;
    if word_number.contains(" and ") {
        let parts = word_number.splitn(2, " and ").collect::<Vec<&str>>();
        word_numbers_mapped_option_possible = parts
            .get(0)
            .unwrap()
            .split_whitespace()
            .map(|number| exchange_word_for_number(number))
            .collect::<Vec<Option<u64>>>();
        let numbers_after_and = match parts.get(1) {
            Some(smth) => smth,
            None => "",
        };
        word_numbers_mapped_option_possible.extend(
            numbers_after_and
                .split_whitespace()
                .map(|number| exchange_word_for_number(number)),
        );
    } else {
        word_numbers_mapped_option_possible = word_number
            .split_whitespace()
            .map(|number| exchange_word_for_number(number))
            .collect::<Vec<Option<u64>>>();
    }
    drop(word_number);
    let mut non_multiplier_indexes: Vec<usize> = Vec::new();
    for (index, number) in word_numbers_mapped_option_possible.iter().enumerate() {
        let number = match number {
            Some(number) => {
                word_numbers_mapped.push(*number);
                *number
            }
            None => return Err(BadRequest),
        };
        if is_non_multiplier_number(number) && index != 0 {
            non_multiplier_indexes.push(index)
        }
    }
    drop(word_numbers_mapped_option_possible);
    let mut elements_removed: usize = 0;
    let mut numbers_to_add = Vec::new();
    for mut index in non_multiplier_indexes {
        index -= elements_removed;
        let mut temp_number: u64 = 1;
        for i in 0..index {
            temp_number = temp_number
                .checked_mul(word_numbers_mapped[i])
                .ok_or(BadRequest)?;
        }
        numbers_to_add.push(temp_number);
        elements_removed += index;
        word_numbers_mapped.drain(0..index);
    }
    if !word_numbers_mapped.is_empty() {
        if is_non_multiplier_number(word_numbers_mapped[0]) {
            if word_numbers_mapped.len() > 1 {
                let mut temp_number: u64 = 1;
                for number in word_numbers_mapped {
                    temp_number = temp_number.checked_mul(number).ok_or(BadRequest)?;
                }
                number = number.checked_add(temp_number).ok_or(BadRequest)?;
            } else {
                number = number
                    .checked_add(word_numbers_mapped[0])
                    .ok_or(BadRequest)?;
            }
        } else {
            for num in word_numbers_mapped {
                number = number.checked_add(num).ok_or(BadRequest)?;
            }
        }
    }
    for num in numbers_to_add {
        number = number.checked_add(num).ok_or(BadRequest)?;
    }
    Ok(number)
}
fn exchange_word_for_number(number_word: &str) -> Option<u64> {
    if number_word.contains('-') {
        let words = number_word.split('-');
        let mut temp_number: u64 = 0;
        for word in words {
            temp_number += match_word_to_number(word)?;
        }
        Some(temp_number)
    } else {
        match_word_to_number(number_word)
    }
}

fn match_word_to_number(number_word: &str) -> Option<u64> {
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
        "million" => Some(1_000_000),
        "billion" => Some(1_000_000_000),
        _ => None,
    }
}

fn is_non_multiplier_number(number: u64) -> bool {
    number < 100
}
