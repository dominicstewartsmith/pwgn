
use std::{env::args};
use pwgn::{parse_options_from_args, generate_password_from_options};
fn main() {
    let arguments: Vec<String> = args().skip(1).collect();
    let options = parse_options_from_args(arguments);

    let generated = generate_password_from_options(&options);

    println!("{}", generated);
}