use std::io::Write;
use std::io;

fn main(){
    let mut counter = 0;

    let results = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("resut is {results}");


    'main_loop: loop {
        print!("enter a number to count till | press n to end: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
        .expect("failed to read the message");
        

        
        if input.trim().eq("n") {
            break 'main_loop; // breaks the main loop
        }
        let _target: u32;
        match input.trim().parse() {
            Ok(num) => { _target = num; },
            Err(_) => {
                println!("invalid number. try again...");
                continue;
            },
        }

        if _target > 10 {
            println!("number should be max 10");
            continue;
        }

        let mut count: u32 = 1;
        let mut num_arr= [0; 10];
        while count <= _target {
            println!("{}", count);
            let index: usize = count.try_into().expect("error in count");
            num_arr[index - 1] = count;
            count += 1;
        }
        
        println!("{:?}", num_arr);
    }
}