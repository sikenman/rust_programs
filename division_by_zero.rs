
use std::io;

fn main() {
    println!("⚠️ Program to demo Division by Zero");
    println!("=====================================");
    

    let x = ask_numbers("first".to_string());
    let y = ask_numbers("second".to_string());

    if y == 0 {
        panic!("Division by zero");
    }

    let z = x / y;
    println!("{}", z);
}

fn ask_numbers(my_num: String) -> i32 {
    println!("Enter {my_num} number: ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Invalid input");
    return num;
}
