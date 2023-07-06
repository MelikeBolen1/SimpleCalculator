use std::io;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operation: Operation, num1: f64, num2: f64) -> Option<f64> {
    match operation {
        Operation::Add => Some(num1 + num2),
        Operation::Subtract => Some(num1 - num2),
        Operation::Multiply => Some(num1 * num2),
        Operation::Divide => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
    }
}

fn main() {
    // Prompt the user to input the first number
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid number");

    // Prompt the user to input the operation
    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation: Operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Prompt the user to input the second number
    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid number");

    // Call the calculate function with the created Operation enum instance
    if let Some(result) = calculate(operation, first_number, second_number) {
        println!("Result: {}", result);
    } else {
        println!("Cannot divide by zero");
    }
}
