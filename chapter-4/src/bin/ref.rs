fn main(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("length of '{s1}' is {len}");

    let mut str1 = String::from("this is a mutable string");
    println!("string before: {str1}");

    /*
        &mut means mutable reference
        the value of that reference also has to be mutable.

        however, there is 1 restriction.
    */
    change(&mut str1);
    println!("string after: {str1}");

    
    
    // let r1 = &mut str1;
    // let r2 = &mut str1;
    /*
    using both mutable references at same time is not allowed.
    but we can have the references.
    borrowing more than once is not allowed.
    
    this prevents the race condition type error.
    because it eliminates the chance of data modification at same time.
    
    so its better to use different scope for same purpose.
    */
    // println!("{r1} , {r2}");
    
    {
        let r1 = &mut str1;
        println!("r1: {r1}");
    }
    
    {
        let r2 = &mut str1;
        println!("r2: {r2}");
    }

    /*
        lets try with immutable reference too.
    */
    let ir1 = &str1;
    let ir2 = &str1;
    println!("{ir1} , {ir2}"); // this wont cause any 
    /*
    but adding a mutable reference with another refernce will cause problem.
    */
    let mr1 = &mut str1;
    // println!("{ir1} , {ir2}"); // this will give error OR
    // println!("{ir1} , {mr1}"); // this will give error OR
    println!("{mr1}"); // no issue

    /*
        reference scope starts when it is first introduced.
        continues through the last time that reference is used.
        need to make sure there is no overlapping between mutable and immutable reference.
    */

    /*
        dangling reference is also not allowed in rust :)
    */
}

/*
    using reference will not take ownership.
    we can't reassign or modify the value as it is not owned by s.
    this is borrowing.

    but we can use mutable references to modify.
*/
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change (s: &mut String) {
    s.push_str(" - modified");
}