use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let file_name = "hello.txt";
    let file_result = File::open(&file_name);

    let mut file = match file_result {
        Ok(ff) => ff,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fcs) => fcs,
                Err(e) => panic!("failed to create new file: {e:?}"),
            },
            _ => panic!("failed to open the file: {error:?}")
        }
    };

    let mut file_content = String::new();
    let file_read_result = file.read_to_string(&mut file_content);
    
    match file_read_result {
        Ok(_) => println!("{file_content}"),
        Err(error) => println!("error occured while reading file: {error:?}"),
    };
}
