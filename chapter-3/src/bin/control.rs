use std::io::{self, Write};

fn main(){
    print!("enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");

    match input.trim().parse() {
        Ok(num) => {
            even_odd(num);

            if is_even(num) {
                println!("is_even: even");
            }
            else {
                println!("is_even: odd");
            }
        },
        Err(_) => println!("invalid number"),
    };
}

fn even_odd (num: i32) {
    if (num % 2) == 0 {
        println!("even_odd: even");
    }
    else {
        println!("even_odd: odd");
    }
}

fn is_even (num: i32) -> bool {
    if num % 2 == 0 { true } else { false }
    /*
        writing a ; in the end makes it statement.
        statements don't return value. expression do.
        calling a function is expression, and so is the conditionals, if we don't write ;
    */
}