use rand::{self, seq::SliceRandom, Rng};
use std::{env::args};

const UPPERCASE_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const LOWERCASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIALS: [char; 10] = ['$', '@', '*', '&', '=', '%', '+', ';', '^', '-'];

#[derive(Debug)]
struct Options {
    default_values: bool,
    using_custom_format: bool,
    using_custom_length: bool,
    output_length: usize,
    randomise_output: bool,
    format: String,
}

impl Options {
    fn default () -> Options {
        Options {
            default_values: true,
            using_custom_format: false,
            using_custom_length: false,
            output_length: 12,
            randomise_output: false,
            format: String::from("llluuunnns")
        }
    }
}

fn validate_format(format: &String) -> bool {

    // Refactor to Result<>?

    // Format string must contain at least one of l, u, n, s
    // Min length 6
    // Max length 255

    let valid = (format.len() >= 6 && format.len() <= 255) && (format.contains('l') || format.contains('u') || format.contains('n') || format.contains('s'));
    //println!("Received format {0}, valid: {1}", format, valid);

    if !valid {
        println!("Invalid custom format string ({}). Reverting to default.", format);
        println!("Must be at least 6, at most 255 characters, and include at least one of: l(owercase), u(ppercase), n(umber), s(pecial).");
    } else {
        println!("Custom format was valid.");
    }

    valid
}

fn parse_options_from_args (arguments: Vec<String>) -> Options {
    let mut options = Options::default();

    let mut accept_format = false;

    for arg in arguments {

        match arg.as_str() {
            "-r" => options.randomise_output = true,
            "-f" => accept_format = true,            
            other => {
                if accept_format {
                    let input_format = other.chars().filter(|&c| (c as u32) >= 32 && (c as u32) <= 126).collect::<String>(); // Filter non-printable ASCII;

                    if validate_format(&input_format){
                        options.format = input_format;
                        options.using_custom_format = true;
                        options.default_values = false;
                    }

                    accept_format = false;
                    continue;
                }
                if let Ok(parsed_value) = other.parse::<usize>() {
                    options.output_length = if parsed_value < 6 { println!("Length too small, changing to 6."); 6 } else if parsed_value > 255 { println!("Length too long, changing to 255."); 255 } else { parsed_value }; // min 6, max 255
                    options.default_values = false;
                    options.using_custom_length = true;
                    options.format = generate_format_from_custom_length(options.output_length);
                }

            }
        }
    }

    // Only allow either custom format, or custom length.
    // If we received a custom format, it takes priority.
    if options.using_custom_format && options.using_custom_length {
        println!("A custom format takes priority over a custom length, so your custom length will be ignored.");
        options.using_custom_length = false;
    }

    if options.using_custom_format { options.output_length = options.format.len(); }

    if options.randomise_output {
        let mut rng = rand::rng();

        let mut copy: Vec<char> = options.format.clone().chars().collect();
        copy.shuffle(&mut rng);
        options.format = copy.into_iter().collect();
    }

    options
}
fn main() {
    let arguments: Vec<String> = args().skip(1).collect();
    let options = parse_options_from_args(arguments);

    let generated = generate_password_from_options(&options);

    //println!("{:?}", options);
    println!("{}", generated);
}

fn generate_password_from_options (options: &Options) -> String {
    let mut output = String::new();

    for c in options.format.chars() {
        output.push(match_format_char_to_replacement(c));
    }    

    output
}


fn generate_format_from_custom_length (length: usize) -> String {
    let mut output = String::new();
    
    // Hard limit max special characters and numbers depending on password length
    let mut max_inserts = if length <= 8 { 1 } else { 2 };

    let half = (length - max_inserts) / 2;

    for num in 0..length-max_inserts {
        output.push(if num < half { 'l' } else { 'u' });
    }

    while max_inserts > 0 {
        output.push('n');
        output.push('s');
        max_inserts -= 1;
    }

    output
}

fn match_format_char_to_replacement(char: char) -> char {
    match char {
        'l' => pick_char(&LOWERCASE_LETTERS),
        'u' => pick_char(&UPPERCASE_LETTERS),
        'n' => pick_char(&NUMBERS),
        's' => pick_char(&SPECIALS),
        _ => char,
    }
}

fn pick_char(charset: &[char]) -> char {
    let mut rng = rand::rng();
    let index = rng.random_range(0..charset.len());

    charset[index]
}
