use std::io::{self, Write};
use std::process::exit;
mod condition;
mod eval;

fn main() {
    println!("EsoCalc 1.0");
    loop {
        let expr = prompt().expect("Unable to read prompt");

        if expr.chars().nth(0).unwrap() == '?' {
            println!("{}", condition::eval(&expr)); // If it is condition...
        } else {
            match &expr as &str {
                "quit" | "exit" | "q" => exit(1),           // Quit
                "version" | "ver" => version(),             // Print version
                _ => println!("{}", eval::seq_read(&expr)), // Eval expression
            }
        }
    }
    //
}

fn prompt() -> Result<String, ()> {
    print!("> ");
    std::io::stdout().flush().expect("Unable to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin");
    Ok(String::from(input.trim())) // Return without trailing '\n'
}

fn version() {
    println!("EsoCalc 1.0");
}
