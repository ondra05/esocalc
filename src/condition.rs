use crate::eval;

pub fn eval<'a>(expr: &'a str) -> u16 {
    let mut to_compare = true;
    let mut exprv: Vec<char> = expr.chars().collect();
    //if exprch.nth(0) != '?' {0}
    if exprv[1] == '!' {
        to_compare = false;
    }
    exprv.pop();
    if to_compare {
        exprv.drain(0..=1);
    } else {
        exprv.drain(0..=2);
    }
    let spl_str: String = exprv.iter().collect(); // Stringified vector
    let spl: Vec<&str> = spl_str.split(")(").collect(); // Splitting to parts
    let condition: Vec<&str> = spl[0].split(",").collect(); // Extract condition
    // ^ Indexes: 0 = Condition, 1 = Expected result ^
    
    let evaled = eval::seq_read(condition[0]);
    let result = eval::seq_read(spl[1]);
    if (evaled == condition[1].parse::<u16>().unwrap()) == to_compare {
        return result;
    }
    0
}
