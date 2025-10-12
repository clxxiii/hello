/**
 * This is a basic implementation of merge sort written in rust.
 * I built this first to use it as a reference when writing my rabbitmq version
 */
use std::io::Write;

fn main() {
    print!("Enter several space-separated numbers on one line: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // This is a "pretty" solution but it is unbelievably slow
    let mut numbers: Vec<i32> = input
        .split(" ")
        .map(|s| s.parse::<i32>().map(|i| Some(i)).unwrap_or(None))
        .filter(|o| o.is_some())
        .map(|o| o.expect("Empty elements have already been filtered out"))
        .collect();

    println!("Unsorted: {numbers:?}");
    numbers = mergesort(numbers);
    println!("Complete: {numbers:?}");
}

pub fn mergesort(arr: Vec<i32>) -> Vec<i32> {
    // Base Case
    if arr.len() <= 1 {
        return arr;
    }

    // Recursively run on array halves
    let midpoint: usize = arr.len() / 2;
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();
    arr[..midpoint].clone_into(&mut arr1);
    arr[midpoint..].clone_into(&mut arr2);
    arr1 = mergesort(arr1.into());
    arr2 = mergesort(arr2.into());

    let result_len = arr1.len() + arr2.len();
    let mut result = Vec::new();
    let mut arr1pos = 0;
    let mut arr2pos = 0;
    for _i in 0..result_len {
        let num1 = arr1.get(arr1pos).unwrap_or(&std::i32::MAX);
        let num2 = arr2.get(arr2pos).unwrap_or(&std::i32::MAX);

        if *num1 < *num2 {
            result.push(*num1);
            arr1pos += 1;
        } else {
            result.push(*num2);
            arr2pos += 1;
        };
    }

    return result;
}

#[test]
fn test_1000000_random() {
    let len: usize = 1_000_000;

    let mut shuffled: Vec<i32> = Vec::new();
    for _ in 0..len {
        shuffled.push(rand::random::<i32>());
    }

    let sorted = mergesort(shuffled);

    // Assert each number is smaller than the last
    for i in 1..len {
        let i1 = sorted[i - 1];
        let i2 = sorted[i];

        assert!((i2 - i1) >= 0)
    }
}
