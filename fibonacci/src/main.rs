use std::io;

fn main() {
    println!("Input number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: u32 = input.trim().parse().expect("Please type a number!");
    let result = fibonacci(number);
    match number % 10 {
    1 => println!("The {number}st Fibonacci number is {result}"),
    2 => println!("The {number}nd Fibonacci number is {result}"),
    3 => println!("The {number}rd Fibonacci number is {result}"),
    _ => println!("The {number}th Fibonacci number is {result}"),
    }
}

fn fibonacci(n: u32) -> u32 {
    if n > 2 {
        fibonacci(n-1)+fibonacci(n-2)
    } else {
        1
    }
}