use rand::{self, Rng, seq::SliceRandom};

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
pub struct Options {
    default_values: bool,
    using_custom_format: bool,
    using_custom_length: bool,
    output_length: usize,
    randomise_output: bool,
    format: String,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            default_values: true,
            using_custom_format: false,
            using_custom_length: false,
            output_length: 12,
            randomise_output: false,
            format: String::from("llluuunnns"),
        }
    }
}

fn validate_format(format: &str) -> Result<bool, &str> {
    let len = format.len();
    let too_short = len < 6;
    let too_long = len > 255;
    let contains_format_char = format.contains('l')
        || format.contains('u')
        || format.contains('n')
        || format.contains('s');

    if too_short {
        return Err("The custom format was too short. Reverting to default.");
    }
    if too_long {
        return Err("The custom format was too long (max 255). Reverting to default.");
    }
    if !contains_format_char {
        return Err(
            "The custom format was missing at least one format character (l, u, n, s). Reverting to default.",
        );
    }

    Ok(true)
}

pub fn parse_options_from_args(arguments: Vec<String>) -> Options {
    let mut options = Options::default();

    let mut listen_for_custom_format = false;

    for arg in arguments {
        match arg.as_str() {
            "-r" => options.randomise_output = true,
            "-f" => listen_for_custom_format = true,
            other => {
                if listen_for_custom_format {
                    let input_format = other
                        .chars()
                        .filter(|&c| (c as u32) >= 32 && (c as u32) <= 126)
                        .collect::<String>(); // Filter non-printable ASCII;
                    let format_valid = validate_format(&input_format);

                    match format_valid {
                        Ok(_) => {
                            options.format = input_format;
                            options.using_custom_format = true;
                            options.default_values = false;
                        }
                        Err(message) => {
                            println!("{}", message);
                            options = Options::default();
                        }
                    }

                    listen_for_custom_format = false;
                } else if let Ok(parsed_value) = other.parse::<usize>() {
                    options.output_length = if parsed_value < 6 {
                        println!("Length too small, changing to 6.");
                        6
                    } else if parsed_value > 255 {
                        println!("Length too long, changing to 255.");
                        255
                    } else {
                        parsed_value
                    }; // min 6, max 255
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
        println!(
            "A custom format takes priority over a custom length, so your custom length will be ignored."
        );
        options.using_custom_length = false;
    }

    if options.using_custom_format {
        options.output_length = options.format.len();
    }

    if options.randomise_output {
        let mut rng = rand::rng();

        let mut copy: Vec<char> = options.format.clone().chars().collect();
        copy.shuffle(&mut rng);
        options.format = copy.into_iter().collect();
    }

    options
}

pub fn generate_password_from_options(options: &Options) -> String {
    let mut output = String::new();

    for c in options.format.chars() {
        output.push(match_format_char_to_replacement(c));
    }

    output
}

fn generate_format_from_custom_length(length: usize) -> String {
    let mut output = String::new();

    // Hard limit max special characters and numbers depending on password length
    let max_inserts = if length <= 8 { 1 } else { 2 };

    let half = (length - max_inserts) / 2;

    for num in 0..length - (max_inserts * 2) {
        output.push(if num < half { 'l' } else { 'u' });
    }

    output += if max_inserts == 1 { "ns" } else { "nnss" };

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

#[cfg(test)]
mod tests {
    use crate::{generate_format_from_custom_length, validate_format};

    #[test]
    fn generates_format_from_custom_length() {
        let pw = generate_format_from_custom_length(12);
        assert_eq!(12, pw.len());
    }

    #[test]
    fn rejects_invalid_format() {
        let too_short = String::from("l");
        let too_long = String::from("l").repeat(256);
        let no_format_char = String::from("kkkkkkkkk");

        assert_eq!(
            validate_format(&too_short),
            Err("The custom format was too short. Reverting to default.")
        );
        assert_eq!(
            validate_format(&too_long),
            Err("The custom format was too long (max 255). Reverting to default.")
        );
        assert_eq!(
            validate_format(&no_format_char),
            Err(
                "The custom format was missing at least one format character (l, u, n, s). Reverting to default."
            )
        );
    }
}
