use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let op: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let num1 = first.parse::<f32>().unwrap();
    let num2 = second.parse::<f32>().unwrap();

    let result = operate(num1, num2, op);
    println!("{} {} {} = {}", first, op, second, result);
}

fn operate(num1: f32, num2: f32, operator: char) -> f32 {
    match operator {
        'x' | '*' => num1 * num2,
        '/' => num1 / num2,
        '+' => num1 + num2,
        '-' => num1 - num2,
        _ => panic!("Invalid Operator!"),
    }
}
