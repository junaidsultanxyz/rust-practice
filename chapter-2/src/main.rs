/*
 Import input output (io) library from Standard (std) library.
 If not importing, can use scope operator to directly call functions.
 like in C/C++

 use keyword lets you import non prelude libraries or functions.

 prelude: already included in every rust project, like println.
*/
use std::io;

/*
    Import ordering library from comparison library, from standard library
*/
use std::cmp::Ordering;


/*
    Import random number generator (Rng) from Random (rand) library.
    The `gen_range(start..=end)` method is defined in Rng.
*/
use rand::Rng;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("secret number is : {secret_number}");

    let mut guess_count = 0;

    loop {

        println!("Guess the number: ");
        
        /*
            Define mutable vairable using `mut`. Variables without mut are
            by default immutable.

            `String::new()` creates an empty string.
        */
        let mut guess = String::new();
        
        
        /*
            Standard input. Use `read_line` to take a line input.
            Pass in a mutable reference. References are by default immutable.
            So, passing &guess will not compile.
            
            &mut means mutable reference. & indicates reference.
            
            read_line returns a Result value which is an enum.
            Results can be: [Ok, Err]
            Ok: contains successfully generated value
            Err: means operation failed, contains information about how or
            why the operation failed

            Result value `expect` method. If Result is Err, program will crash
            and display the message passed as argument.

            Not writing expect method will give a warning, but compile the code.
            Warning is to handle an expected error.
        */
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    
        /*
            guess: u32 defines guess as an unsigned 32 bit integer.
            trim() removes the whitespace from start and end. (string method).
            parse() parses the value to appropriate data type, u32 in this case.
            parse returns a Result, which we can use expect like above to handle exception.

            Or, we can use the status itself to define custom behavior for each result.
            Now, we are matching the expression.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid number");
                continue;
            },
        };


        /*
            {} is a placeholder.
            Putting a variable inside {} like {guess} prints its value.
            To evalute an expression, use empty placeholder like following:

            println!("1 + 1 = {}", 1 + 1);
            
            or 
            
            let x = 5;
            let y = 10;
            println!("x = {x} and y + 2 = {}", y + 2);
        */
        println!("you guessed: {guess}");
        
        
        /*
            Compare guess and secret_number. pattern/type must match.
        */

        guess_count += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win. total guesses = {guess_count}");
                break;
            }
        }
    }
}
