use std::io::{self, Write};

fn main(){
    print!("nth fibonacci: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("failed to read input");

    match input.trim().parse() {
        Ok(num) => {
            let fibo = n_fibo(num);
            println!("{num} fibonacci = {fibo}");
        }
        Err(_) => {
            println!("invalid number format");
        },
    }
}

fn n_fibo (n: u32) -> u32 {
    if n == 0 | 0 { 0 }
    else if n == 1 { 1 }
    else if n >= 2 {
        let mut remaining = n - 1;
        let mut first: u32 = 0;
        let mut second: u32 = 1;

        let mut next: u32 = 0;
        while remaining > 0 {
            next = first + second;
            first = second;
            second = next;
            remaining -= 1;
        }

        next
    }
    else {
        0
    }
}