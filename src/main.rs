use rand::{self, Rng};
use std::env::args;

const UPPERCASE_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const LOWERCASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIALS: [char; 6] = ['$', '@', '*', '&', '=', '%'];

fn main() {
    let arguments: Vec<String> = args().collect();

    let format: &str = if arguments.len() > 1 {
        &arguments[1]
    } else {
        "llluuunnns"
    };

    let mut output = String::new();

    for x in format.chars() {
        output.push(match x {
            'l' | 'L' => pick_char(&UPPERCASE_LETTERS),
            'u' | 'U' => pick_char(&LOWERCASE_LETTERS),
            'n' | 'N' => pick_char(&NUMBERS),
            's' | 'S' => pick_char(&SPECIALS),
            _ => x,
        });
    }

    println!("{}", output);
}

fn pick_char(charset: &[char]) -> char {
    let mut rng = rand::rng();
    let index = rng.random_range(0..charset.len());

    charset[index]
}
