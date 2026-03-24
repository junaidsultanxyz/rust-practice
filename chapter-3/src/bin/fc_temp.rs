use std::io::{self, Write};

fn main() {
    let mut input = String::new(); // Declare once outside the loop

    loop {
        println!("\n.----- Temperature Conversion -----. ");
        println!("|     1. Celsius to Fahrenheit     |");
        println!("|     2. Fahrenheit to Celsius     |");
        println!("|     3. Exit                      |");
        println!("'----------------------------------'");

        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: u8 = match input.trim().parse() {
            Ok(3) => break,
            Ok(num) if num == 1 || num == 2 => num,
            _ => {
                println!("[ERROR] Please enter 1, 2, or 3");
                continue;
            }
        };

        print!("Enter Temperature: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let val: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[ERROR] Invalid number format");
                continue;
            }
        };

        let result = if choice == 1 {
            celsius_to_fahrenheit(val)
        } else {
            fahrenheit_to_celsius(val)
        };

        println!(".----------------------------------.");
        println!("|     Result: {result:.2}          |");
        println!("'----------------------------------'");
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}