
/*
    consts are always immutable. it doesn't allow `mut` to be written with it.
    you must annotate a data type with const.

*/
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/*
    can't use let in global referece.
    use static instead.
    if using static, have to annotate a data type.
*/
static ANOTHER_CONST: u32 = 67;

fn main() {
    /*
        By default, all variables are immutable.
        If we try to change immutable variables, it gives compile error.
    */
    // let a = 5;
    // a = 10;

    /*
        To make mutable variables, mention `mut` before the variable name.
    */
    let mut x= 5;
    println!("the value of x = {x}");
    x = 10;
    println!("the value of x = {x}");
    
    /*
        Letting the same immutable variable again is called shadowing.
    */
    // let y = 10; // unused warning
    let y = 5;
    let y = y + 67;

    {
        let y = y / 2; // stays within the inner scope only. inner shadowing.
        println!("value of y = {y}");
    }
    println!("value of y = {y}"); // resets back to outer states. outershadowing.
    
    println!("3 hours in seconds = {THREE_HOURS_IN_SECONDS}");
    println!("another const = {ANOTHER_CONST}");


    /*
        We can also change the data type of an immutable variable.
    */
    let spaces = "     ";
    let spaces = spaces.len();
    
    println!("spaces = {spaces}");

    /*
        we cannot change type of a mutable variable.
    */
    // let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len();
}
