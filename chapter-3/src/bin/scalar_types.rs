fn main() {
    let x = 2.0;
    let y: f32 = 3.5;

    let sum = x + y;
    let subtract = x - y;
    let multiply = x * y;
    let divide = x / y;
    let modulus = x % y;

    println!("sum = {sum}");
    println!("subtract = {subtract}");
    println!("multiply = {multiply}");
    println!("divide = {divide}");
    println!("modulus = {modulus}");


    /*
        writing 1 or 0 instead of true/false wont compile.
    */
    let is_true: bool = true;
    let is_false = false;

    println!("true? = {is_true}");
    println!("false? = {is_false}");

    /*
        UTF-8
        Writing more than 1 character wont compile.
    */
    let c = 'c';
    let c_a: char = 'C';

    println!("character: {c} and {c_a}");
}