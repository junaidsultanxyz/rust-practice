pub fn part2() {
    let s1: String = String::from("string 1"); //immutable string on heap
    println!("s1 {s1}");
    
    let s2: &str = "string 2"; // immutable string slice on stack
    println!("s2 {s2}");
    
    let s3: String = s2.to_string() + &s1;
    let s3_2: String = s1 + &s2;
    // fn add(self, s: &str) -> String {

    println!("s3 {s3}");
    println!("s2 {s2}");
    println!("s3_2 {s3_2}");
    // println!("s1 {s1}"); // value of s1 moved so the old reference is no longer valid

    let hello = String::from("السلام عليكم");
    let hello = String::from("こんにちは");
    println!("{hello}");

    let mut s4 = String::from("hello, ");
    s4.push_str("world");
    println!("s4 {s4}");

    let mut s5 = "lo".to_string();
    s5.push('l');
    println!("s5 {s5}");

    let s = format!("{s2}-{s3}-{s3_2}-{s4}-{s5}"); // does not take ownership of any string
    println!("{s}");


    let example = String::from("hello"); // this string is 5 bytes
    let example2 = String::from("ㄟ(≧◇≦)ㄏ"); // this string is 5*2 bytes (UTF-8) 

    let index_char = &example[0..2];
    let index_char2 = &example2[0..3]; // can crash with 0..2 as it wont be a complete char for the example2

    println!("{index_char}");
    println!("{index_char2}");


    for (index,c) in example2.chars().enumerate() {
        println!("{index} - {c}");
    }
    println!();
    for (index,b) in example2.bytes().enumerate() {
        println!("{index} - {b}");
    }

    // check the output to see the difference between .chars() and .bytes() for this exmaple
}
