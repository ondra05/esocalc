use std::convert::TryInto;

struct Memory {
    power: u16,
    number: u16,
    result: u16,
}
struct CurrentState {
    symbol: char,
    command: char,
    args: u8,
    state: i8,
    memory: Memory,
}

impl CurrentState {
    fn reset(&mut self) {
        self.symbol = '_';
        self.command = '_';
        self.args = 0;
    }

    fn clear(&mut self) {
        self.memory.number = 0;
        self.memory.power = 0;
    }
}

pub fn seq_read<'a>(expr: &'a str) {
    let mut cs = CurrentState {
        symbol: '_',
        command: '_',
        args: 0,
        state: 0,
        memory: Memory {
            power: 0,
            number: 0,
            result: 0,
        },
    };
    for i in expr.chars() {
        match cs.state {
            0 => match i { // Symbol part
                '%' | '^' => cs.symbol = i,
                // Special symbols:
                ';' => {
                    cs.memory.result += cs.memory.number.pow(cs.memory.power as u32);
                    println!("{}", cs.memory.result)
                },
                '@' => {
                    cs.memory.result += cs.memory.number.pow(cs.memory.power as u32);
                    cs.reset();
                    cs.clear();
                    cs.state = -1
                },
                '&' => {
                    cs.memory.result += cs.memory.number.pow(cs.memory.power as u32);
                    cs.state = -1
                },
                _ => println!("Unknown symbol: {}", i),
            },
            1 => match i { // Command part
                '+' | '-' | '*' | '/' => cs.command = i,
                _ => {}
            },
            2 => {
                cs.args = i.to_digit(10).unwrap().try_into().unwrap(); // Add argument
                eval_sub(&mut cs) // Create sum
            },
            _ => println!("How did we got there?!"),
        }
        cs.state += 1;
    }
}

fn eval_sub<'a>(cs: &mut CurrentState) {
    match cs.symbol {
        '%' => cs.memory.number += cs.args as u16,
        '^' => cs.memory.power += cs.args as u16,
        _ => {}
    }
    cs.reset();
    cs.state = -1; // Reset to -1 (because then it will raise by 1 to 0 by state incrementer)
}
