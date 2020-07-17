struct CurrentState<'a> {
    symbol: char,
    command: char,
    args: i8,
    state: i8,
}

pub fn seq_read<'a>(expr: &'a str) {
    let mut cs = CurrentState {
        symbol: '_',
        command: '_',
        args: 0,
        state: 0,
    };
    for i in expr.chars() {
        match cs.state {
            0 => match i {
                '%' | '^' => cs.symbol = '%',
                _ => println!("Unknown symbol: {}", i),
            },
            1 => match i {
                '+' | '-' | '*' | '/' => cs.command = i,
                _ => {}
            },
            2 => cs.args = i.to_digit(0).unwrap() as i8,
            _ => println!("How did we got there?!"),
        }
    }
    cs.state += 1;
}

fn eval<'a>(cs: &CurrentState<'a>) {
    let mut power = 0;
    let mut number = 0;
    let mut result = 0;
    match cs.symbol {
        '%' => number += cs.args,
        '^' => power += cs.args,
    }
}
