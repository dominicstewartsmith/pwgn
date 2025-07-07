use rand::{self, seq::SliceRandom, Rng};
use std::{char::from_u32, cmp, env::args, option};

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

// todo - geneate from ascii char code instead of hardcoded arrays
#[derive(Debug)]
struct Options {
    using_custom_format: bool,
    using_custom_length: bool,
    output_length: usize,
    randomise_output: bool,
    format: String,
}

impl Options {
    fn default () -> Options {
        Options {
            using_custom_format: false,
            using_custom_length: false,
            output_length: 12,
            randomise_output: false,
            format: String::from("llluuunnns")
        }
    }
}

fn validate_format(format: &String) -> bool {

    // Refactor to Result<>

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
                    let input_format = String::from(other).chars().filter(|&c| (c as u32) >= 32 && (c as u32) <= 126).collect::<String>(); // Filter non-printable ASCII);

                    if validate_format(&input_format){
                        options.format = input_format;
                        options.using_custom_format = true;
                    }

                    accept_format = false;
                    continue;
                }
                if let Ok(parsed_value) = other.parse::<usize>() {
                    options.output_length = if parsed_value < 6 { 6 } else if parsed_value > 255 { 255 } else { parsed_value }; // min 6, max 255
                    options.using_custom_length = true;
                    if !options.using_custom_format { options.format.clear(); }
                }

            }
        }
    }

    // Only allow either custom format, or custom length.
    // If we received a custom format, it takes priority.
    if options.using_custom_format { options.output_length = options.format.len(); }

    options
}
fn main() {
    let arguments: Vec<String> = args().skip(1).collect();
    let options = parse_options_from_args(arguments);

    generate_password_from_options(&options);

    println!("{:?}", options);


    


    // let res = generate_valid(&mut options);

    // if options.randomise_output {
        
    //     let mut rng = rand::rng();
    //     let mut copy: Vec<char> = res.clone().chars().collect();
    //     copy.shuffle(&mut rng);
    //     println!("{0}, {1}", res, copy.into_iter().collect::<String>());
    // };

    // println!("{0}, {1}", res, options.randomise_output);
}

fn generate_password_from_options (options: &Options) -> String {
    let mut output = String::new();

    if options.using_custom_format {
        for c in options.format.chars() {
            output.push(match_format_char_to_replacement(c));
        }
    }

    output
}

fn generate_random_format (length: usize) {
    // Hard limit to max 2 special characters and numbers
    let mut rng = rand::rng();

    let mut inserted_numbers = 0;
    let mut inserted_specials = 0;

    let mut output = String::new();

    let mut rand_index = rng.random_range(0..SPECIALS.len());
    output.push(SPECIALS[rand_index]);

    rand_index = rng.random_range(0..NUMBERS.len());
    output.push(NUMBERS[rand_index]);

    let i = 2;
    while i < length {
        let c = unsafe { char::from_u32_unchecked(rng.random_range(65..=122)) };
        output.push(c); // cannot fail
    }



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

fn generate_valid(mut options: &mut Options) -> String {
    let mut generated_password = generate(&mut options);

    while !validate(&generated_password) {
       generated_password = generate(&mut options);
    };

    generated_password
}

fn generate(options: &mut Options) -> String {
    println!("here");

    let mut rng = rand::rng();

    let mut random_output = String::new();

    // If a custom length is selected, randomise output by default
    if options.output_length != 0 {
        options.randomise_output = true;
    };

    if options.randomise_output && options.output_length > 0 {
        for _ in 0..options.output_length {
            let rand = rng.random_range(0..=100);
            match rand {
                0..=25 => random_output.push('l'),
                26..=50 => random_output.push('u'),
                51..=75 => random_output.push('n'),
                76..=100 => random_output.push('s'),
                _ => ()
            }
        }

        options.format = random_output;
        
    }

    let mut output = String::new();

    for x in options.format.chars() {
        output.push(match x {
            'l' => pick_char(&LOWERCASE_LETTERS),
            'u' => pick_char(&UPPERCASE_LETTERS),
            'n' => pick_char(&NUMBERS),
            's' => pick_char(&SPECIALS),
            _ => x,
        });
    }

    output


}
fn validate (generated_password: &String) -> bool {
    // validate occurence of each type
    let (mut upper_found, mut lower_found, mut num_found, mut special_found) = ( false, false, false, false );


    for chr in generated_password.chars() {
        if !upper_found {
            if UPPERCASE_LETTERS.contains(&chr) {
                upper_found = true;
            }
        }

        if !lower_found {
            if LOWERCASE_LETTERS.contains(&chr) {
                lower_found = true;
            }
        }

        if !num_found {
            if NUMBERS.contains(&chr) {
                num_found = true;
            }
        }

        if !special_found {
            if SPECIALS.contains(&chr) {
                special_found = true;
            }
        }
    };

    upper_found && lower_found && num_found && special_found

}
fn pick_char(charset: &[char]) -> char {
    let mut rng = rand::rng();
    let index = rng.random_range(0..charset.len());

    charset[index]
}
