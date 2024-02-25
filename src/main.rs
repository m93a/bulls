use std::io;
use std::io::Write;

const SYMBOLS: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@#&%_+-*/^()[]{}<>:;";

fn main() {
    println!();
    println!("Welcome to Bulls and Cows!");
    let si = io::stdin();
    let mut so = io::stdout();
    let mut read_str = String::new();
    let mut read_int = |msg: &str, default: Option<usize>| -> usize {
        loop {
            println!("{}", msg);
            print!("> ");
            so.flush().unwrap();
            read_str.clear();
            si.read_line(&mut read_str).unwrap();
            println!();
            if read_str.trim() == "" && default.is_some() {
                println!("> {}", default.unwrap());
                return default.unwrap();
            }
            if let Ok(num) = read_str.trim().parse::<usize>() {
                return num;
            }
            println!("Bad input, please enter a positive integer in base 10.");
        }
    };

    let length = read_int(
        "How many symbols do you want to guess? (Default: 4)",
        Some(4),
    );

    let count = read_int(
        "From how many symbols should be in the dictionary? (Default: 10)",
        Some(10),
    );

    if count > SYMBOLS.len() {
        println!("Error: Dictionary cannot be longer than {}.", SYMBOLS.len());
        return;
    }

    let dict = &SYMBOLS[0..count];
 }
