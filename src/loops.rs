use std::io::{self, Write};
fn main() {
    loop {
        print!("Please enter a valid integer: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<i32>() {
                Ok(num) => {
                    println!("You entered the valid integer: {}", num);
                    break;
                }
                Err(_) => {
                    println!("That was not a valid integer. Please try again.");
                }
            },
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}
