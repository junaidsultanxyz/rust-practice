fn main() {
    /*
        this is string literal, stored on stack
     */
    let str_im = "this is in immutable string";

    /*
        String is stored in heap and is mutable
     */
    let mut str_m = String::from("this is a ");
    str_m.push_str("mutable string");
    println!("{str_m}");

    {
        let str_temp = String::from("scroped");
    }
    // str_temp scope ended and no longer valid
    // when this scrope ended, rust called a special function `drop`


    /*
        similarly, the value is dropped after it is reassigned
     */
    let mut str_mut = String::from("value before");
    str_mut = String::from("value after");
    println!("str_mut = {str_mut}");

    let s1 = String::from("example string");
    let s2 = s1.clone(); // deep copy

    let (s3, len) = calculate_length(s2);

    println!("The length of '{s3}' is {len}.");
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}