use std::io;
use std::io::Write;

const SYMBOLS: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@#&%_+-*/^()[]{}<>:;";

fn read(msg: &str) -> String {
    let si = io::stdin();
    let mut so = io::stdout();
    let mut result = String::new();

    println!("{}", msg);
    print!("> ");
    so.flush().unwrap();
    si.read_line(&mut result).unwrap();

    result
}

fn read_int(msg: &str, default: Option<usize>) -> usize {
    loop {
        let read_str = read(msg);
        if read_str.trim() == "" && default.is_some() {
            println!("> {}", default.unwrap());
            return default.unwrap();
        }
        if let Ok(num) = read_str.trim().parse::<usize>() {
            return num;
        }
        println!("Bad input, please enter a positive integer in base 10.");
    }
}

fn main() {
    println!();
    println!("Welcome to Bulls and Cows!");

    println!();
    let length = read_int(
        "How many symbols do you want to guess? (Default: 4)",
        Some(4),
    );

    println!();
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
