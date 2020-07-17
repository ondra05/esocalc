use std::io::{self, Write};
mod eval;
fn main() {
    println!("EsoCalc 1.0");
    let expr = prompt().expect("Unable to read prompt");
    eval::seq_read(&expr);
}

fn prompt() -> Result<String, ()> {
    print!("> ");
    std::io::stdout().flush().expect("Unable to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin");
    Ok(String::from(input.trim()))
}
