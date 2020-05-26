use std::io;

fn main() {
    println!("Enter which number in the Fibonacci sequence you would like returned:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("There was an error with your input");

    let input_number = input.trim().parse::<i32>().unwrap();

    println!("That number in the sequence is {}", fibonacci(input_number));
}

fn fibonacci(number: i32) -> i32 {
    match number {
        0 => 1,
        1 => 1,
        _ => fibonacci(number - 1) + fibonacci(number - 2),
    }
}
