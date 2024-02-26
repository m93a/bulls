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

fn read_int(
  msg: &str,
  default: Option<usize>,
) -> usize {
  let msg = if default.is_some() {
    format!(
      "{} (Default: {})",
      msg,
      default.unwrap()
    )
  } else {
    String::from(msg)
  };

  loop {
    let str = read(&msg);
    if str.trim() == "" && default.is_some()
    {
      println!("> {}", default.unwrap());
      return default.unwrap();
    }
    if let Ok(num) =
      str.trim().parse::<usize>()
    {
      return num;
    }
    println!(
      "Bad input, please enter a positive integer in base 10."
    );
  }
}

fn read_bool(
  msg: &str,
  default: Option<bool>,
) -> bool {
  let msg = format!(
    "{} ({})",
    msg,
    match default {
      Some(true) => "Y/n",
      Some(false) => "y/N",
      None => "y/n",
    }
  );

  loop {
    let str = read(&msg);
    match str.trim() {
      "y" | "Y" | "yes" => return true,
      "n" | "N" | "no" => return false,
      "" if default.is_some() => {
        let def = default.unwrap();
        println!(
          "> {}",
          if def { "y" } else { "n" }
        );
        return def;
      }
      _ => println!(
        "Bad input, please enter either \"y\" or \"n\"."
      ),
    };
  }
}

fn main() {
  println!();
  println!("Welcome to Bulls and Cows!");

  println!();
  let length = read_int(
    "How many symbols do you want to guess?",
    Some(4),
  );

  println!();
  let count = read_int(
    "From how many symbols should be in the dictionary?",
    Some(10),
  );

  if count > SYMBOLS.len() {
    println!(
      "Error: Dictionary cannot be longer than {}.",
      SYMBOLS.len()
    );
    return;
  }

  let dict = &SYMBOLS[0..count];

  println!();
  let repeating = read_bool(
    "Do you want to allow repeating symbols?",
    Some(false),
  );
}
