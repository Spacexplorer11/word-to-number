pub (crate) fn change_word_to_number(word_number: &str) -> u16 {
    let number: u16;

    if word_number.contains("and") {
        let numbers = word_number.split("and");
        let mut temp_number = 1;
        for mut number in numbers {
            if number.starts_with(' ') {
                number = number.strip_prefix(' ').expect("If this gets called something's gone really wrong");
            }
            if number.contains(' ') {
                let numbers = number.split_ascii_whitespace();
                for number in numbers {
                    if !number.eq(" ") {
                        temp_number *= exchange_word_for_number(number);
                    }
                }
            }
            if number.contains('-') {
                let numbers = number.split("-");
                for number in numbers {
                    temp_number += exchange_word_for_number(number);
                }
            }
        }
        number = temp_number;
    }
    else if word_number.contains('-') {
        let numbers = word_number.split('-');
        let mut temp_number = 0;
        for number in numbers {
            temp_number += exchange_word_for_number(number);
        }
        number = temp_number;
    }
    else if word_number.contains("hundred") {
        let mut temp_number = 1;
        if word_number.contains(' ') {
            let numbers = word_number.split_ascii_whitespace();
            for number in numbers {
                if !number.eq(" ") {
                    temp_number *= exchange_word_for_number(number);
                }
            }
        }
        number = temp_number
    }
    else {
        number = exchange_word_for_number(&*word_number);
    }

    println!("Received: {word_number}; Returned: {number}");
    number
}
fn exchange_word_for_number(number_word: &str) -> u16 {
    match number_word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "eleven" => 11,
        "twelve" => 12,
        "thirteen" => 13,
        "fourteen" => 14,
        "fifteen" => 15,
        "sixteen" => 16,
        "seventeen" => 17,
        "eighteen" => 18,
        "nineteen" => 19,
        "twenty" => 20,
        "thirty" => 30,
        "forty" => 40,
        "fifty" => 50,
        "sixty" => 60,
        "seventy" => 70,
        "eighty" => 80,
        "ninety" => 90,
        "hundred" => 100,
        _ => panic!("NUMBER ({number_word}) NOT VALID")
    }
}