fn main(){
    print_hello();
    print_labeled_measurement(5, 'h');
    println!("{}", {add(10, 20)});
}

fn print_hello() {
    println!("hello");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


/*
    expressions: without ; (assignable)
    statements : with ;
*/
fn add (x: i32, y: i32) -> i32 {
    x + y
}