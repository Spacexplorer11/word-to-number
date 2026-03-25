use std::io;

fn main() {
    let mut word_number = String::new();

    println!("Please enter the number in words which you wish to convert:");
    io::stdin().read_line(&mut word_number)
        .expect("Error reading input");

    word_number = String::from(word_number.strip_suffix("\n").unwrap());

    let number: u8;
    if word_number.contains('-') {
        let numbers = word_number.split('-');
        let mut temp_number = 0;
        for number in numbers {
            temp_number += word_to_number(number);
        }
        number = temp_number;
    }
    else {
        number = word_to_number(&*word_number);
    }

    println!("You typed in {word_number} which is actually {number}")
}

fn word_to_number(number_word: &str) -> u8 {
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
        "twenty" => 20,
        "thirty" => 30,
        "forty" => 40,
        "fifty" => 50,
        "sixty" => 60,
        "seventy" => 70,
        "eighty" => 80,
        "ninety" => 90,
        _ => panic!("NUMBER NOT VALID")
    }
}
