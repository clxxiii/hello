use std::env::{args, Args};
use std::fs;

fn main() {
    let mut args: Args = args();
    let file_path = args.nth(1).unwrap();

    let file_result = fs::read_to_string(file_path);

    let file = match file_result {
        Ok(file) => file,
        Err(_) => String::from("Cannot read file."),
    };

    println!("{}", file)
}
