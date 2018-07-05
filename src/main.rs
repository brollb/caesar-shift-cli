extern crate clap;
extern crate isatty;

use clap::{Arg,App};
use std::io::{self, BufRead};

fn main() {
    let has_stdin_pipe = !isatty::stdin_isatty();

    let mut app = App::new("Caesar Shift CLI")
        .version("1.0.0")
        .author("Brian Broll")
        .arg(Arg::with_name("key")
             .help("shift amount to use for the key")
             .takes_value(true)
             .short("k")
             .long("key")
             .default_value("3")
         )
        .arg(Arg::with_name("decrypt")
             .help("decrypt the input message")
             .short("d")
             .long("decrypt")
         );

    if !has_stdin_pipe {
        app = app.arg(Arg::with_name("message")
             .help("Input message to encrypt")
             .required(true)
             .index(1)
        );
    }

    let matches = app.get_matches();

    let mut shift = matches.value_of("key").unwrap().parse::<u8>().unwrap() % 26;
    if matches.is_present("decrypt") {
        shift = 26 - shift;
    }

    if has_stdin_pipe {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Could not read line from standard in");
            encrypt_and_print(&line, shift);
        }
    } else {
        let input_text = matches.value_of("message").unwrap();
        encrypt_and_print(input_text, shift);
    }
}

fn encrypt_and_print(input_text: &str, shift: u8) {
    // convert to a vector of chars and then encrypt each
    let encrypted_chars = input_text.chars()
        .map(|c| {
            if is_letter(c) {
                return shift_letter(c, shift);
            }
            return c;
        });

    encrypted_chars.for_each(|c| {
        print!("{}", c);
    });
    print!("\n");
}

fn is_letter(letter: char) -> bool {
    let num = letter as u8;
    let is_upper = num > 64 && num < 91;
    let is_lower = num > 96 && num < 123;
    is_upper || is_lower
}

fn shift_letter(letter: char, shift: u8) -> char {
    let input = letter as u8;
    let offset = if input > 96 { 96 } else { 64 };

    // Ensure that the number wraps around for the letters
    let encrypted_code = ((input - offset + 26) + shift) % 26 + offset;
    encrypted_code as char
}
