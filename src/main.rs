use std::io;
fn main() {
    println!("❤  Welcome to calculator , When you'r finished you can exit by typing exit ❤");
    loop {
        // Read user input
        let mut input = String::new();
        println!("Enter an Expression to be calculated");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read expression! \nPlease try again");

        // Trim whitespace and check for exist
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Calculator existed , Bye❤.");
            break;
        }

        // Parse the input
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Invalid input . Please enter in the format: <number> <operator> <number>");
            continue; // start the loop again to get valid input from user
        }

        // extract numbers and operator
        let num1: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[0]);
                continue; // start the loop again to get valid input from user
            }
        };
        let operator = parts[1];
        let num2: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[2]);
                continue;
            }
        };

        // Perform the Operation
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed!");
                    continue;
                }
                num1 / num2
            }

            _ => {
                println!("Invalid Operator: {}", operator);
                continue;
            }
        };
        println!("Result: {}", result);
    }
}
