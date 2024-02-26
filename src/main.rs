use rand::prelude::*;
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

fn read_guess(
  msg: &str,
  dict: &[char],
  length: usize,
) -> Vec<char> {
  loop {
    let str = read(msg);
    let chars: Vec<_> =
      str.trim().chars().collect();

    if chars.len() != length {
      println!("Bad input, expected {} symbols, but got {}.", length, chars.len());
      continue;
    }

    for ch in chars.iter() {
      let valid = dict
        .iter()
        .find(|&s| s == ch)
        .is_some();

      if !valid {
        println!("Bad input, character \"{}\" not in the dictionary. Allowed characters: {}", ch, String::from_iter(dict));
        continue;
      }
    }

    return chars;
  }
}

fn generate_secret(
  mut dict: Vec<char>,
  length: usize,
  repeating: bool,
) -> Vec<char> {
  let mut secret =
    Vec::with_capacity(length);

  while secret.len() < length {
    let i = rand::thread_rng()
      .gen_range(0..dict.len());
    secret.push(dict[i]);
    if !repeating {
      dict.remove(i);
    }
  }

  secret
}

struct Matches {
  bulls: usize,
  cows: usize,
}

fn check_matches(
  secret: &[char],
  guess: &[char],
) -> Matches {
  let len = secret.len();

  let mut matches =
    Matches { bulls: 0, cows: 0 };

  let mut secret: Vec<_> = secret
    .iter()
    .map(|&s| Some(s))
    .collect();

  let mut guess: Vec<_> =
    guess.iter().map(|&s| Some(s)).collect();

  for i in 0..len {
    if guess[i] == secret[i] {
      matches.bulls += 1;
      secret[i] = None;
      guess[i] = None;
    }
  }

  for i in 0..len {
    if guess[i].is_none() {
      continue;
    }
    if let Some(si) = secret
      .iter()
      .position(|&s| s == guess[i])
    {
      matches.cows += 1;
      secret[si] = None;
      guess[i] = None;
    }
  }

  matches
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
    "How many symbols should be in the dictionary?",
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
  let dict: Vec<_> = dict.chars().collect();

  println!();
  let repeating = read_bool(
    "Do you want to allow repeating symbols?",
    Some(false),
  );

  let secret = generate_secret(
    dict.clone(),
    length,
    repeating,
  );

  let mut guess_count = 0;
  loop {
    println!();
    let guess = read_guess(&format!("Guess the combination! Guesses so far: {}", guess_count), &dict, length);
    guess_count += 1;

    if guess == secret {
      println!(
        "Correct! It took you {} guesses!",
        guess_count
      );
      break;
    }

    let matches =
      check_matches(&secret, &guess);
    println!(
      "Bulls: {}; Cows {}",
      matches.bulls, matches.cows
    );
  }
}
