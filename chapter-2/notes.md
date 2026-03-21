# Chapter 2 (Project)

built a number guessing game. learnt variables, inputs, reference, printing value and placeholders, library crates.

### Concepts:

- `use <library>` (import a library).
    - Example: `use std::io` . importing input output library from standard library

- `let` (define a variable)
    - `let apples = 5` . immutable variable (default rust behavior)
    - `let mut bananas = 5` . mutable variable
    - `let mut guess = String::new()` . create an empty mutable string

- `io::stdin()` (standard input)
    - `io:stdin().read_line(&mut guess)`. read line. pass in a mutable reference as argument. references are immutable by default, so &guess wouldn't work.
    - `read_line` returns a `Result` type, which is an enum. Possible values of Result is: Ok, Err
    - `Result` has a method called `expect` which uses the return value
    - `expect(<string>)` prints error message if return value is Err. If its Ok, it returns the value.

- `{}` (placeholder)
    - use inside print message.
    - `println!("value = {var}");` : prints the value of the var
    - `println!("1+1 = {}", 1+1);` : here {} will print the calculated expression

- adding rand dependancy in `Cargo.toml`

- `rand::thread_rng().gen_range(start..=end)` : generate a random number in range from stand - end.
    - example: `gen_range(1..=100)` : generates a random number between 1 and 100.

- `let num: u32 = 6969` : defining an annotated type. u32 = unsigned 32 bit integer.

- `str.trim()` : remove whitespace from front and back of string

- `let num: u32 = str.parse()` : parses the string to u32. returns Result.

- custom handling for each result status.
    ```rust
    match <Result> {
        Ok(num) => num,
        Err(_) => {
            println!("invalid number");
            continue; // loop continue
        },
    };
    ```

- pattern matching and comparison
    ```rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => {
            println!("you win");
            break;
        }
    }
    ```

- `loop { /* code */ }` : standard loop. use break and continue inside to control.
