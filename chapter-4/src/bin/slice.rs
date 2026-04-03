fn main(){
    let mut s = String::from("hello world");
    let word = first_word(&s);
    
    println!("{word}");
    s.clear();
    
    /*
    when we cleared s, the word becomes irrelevant as the 5 can't be used for s.
    */
    
    
    let s2 = String::from("hello world");
    let hello = &s2[0..5]; // can also write the range as ..5 (no need to write 0 if it stars from 0)
    let world = &s2[6..11]; // can also write length as ending range or [6..] .
    let s2_complete_slice = &s2[..]; // can also get whole string slice
    println!("{hello}, {world}");
    println!("{s2_complete_slice}");

    println!("{}", get_first_word(&s2));

    // s2.clear(); // now this will throw error
}

fn first_word (s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to bytes

    // iterate over the bytes and return tuples (index, item reference)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // check for byte that represents ' ' (space)
            return i;
        }
    }

    s.len()
}

// taking string slice as parameter and returning a slice
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}