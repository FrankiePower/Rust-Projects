use std::io;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 5 {
            break;
        }

        println!("Enter first number:");

        let mut num1 = String::new();

        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");

        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter second number:");

        let mut num2 = String::new();

        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");

        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input. Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Result: {}", num1.add(num2));
            }
            2 => {
                println!("Result: {}", num1.sub(num2));
            }
            3 => {
                println!("Result: {}", num1.mul(num2));
            }
            4 => {
                println!("Result: {}", num1.div(num2));
            }
            _ => {
                println!("Invalid choice. Please enter a valid choice.");
            }
        }
    }
}
