use std::io;

fn main(){
    let tup: (i32, f64, i32) = (1, 2.0, 3);
    
    let (x, y, z) = tup;
    println!("value of x, y, z = {x}, {y}, {z}");
    println!("tup = {0} , {1}, {2}", tup.0, tup.1, tup.2);



    let arr = [1,2,3,4,5];
    let ar2: [i32; 5] = [1,2,3,4,5];
    let arr3 = [1; 5]; // all 5 indexes init with val 1

    println!("arr[0] = {}", arr[0]);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let index: usize = input.trim().parse().expect("invalid number");

    let element = arr[index];
    println!("The value of the element at index {index} is: {element}");
}