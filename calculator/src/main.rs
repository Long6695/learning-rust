use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first: String = check_error(args.nth(1));
    let operator: char = check_error(args.nth(0)).chars().next().unwrap();
    let second: String = check_error(args.nth(0));

    let first_number = parse_float(&first);
    let second_number = parse_float(&second);
    let result = operate(first_number, second_number, operator);
    println!("{}", output(first_number, operator, second_number, result));
}

fn parse_float(value: &str) -> f32 {
    match value.parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: failed to parse argument as float: {}", value);
            std::process::exit(1);
        }
    }
}

fn check_error(option: Option<String>) -> String {
    match option {
        Some(value) => value,
        None => {
            eprintln!("Error: missing argument.");
            std::process::exit(1)
        }
    }
}

fn operate(first_number: f32, second_number: f32, operator: char) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
      "{} {} {} = {}",
      first_number, operator, second_number, result
    )
  }
