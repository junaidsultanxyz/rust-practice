use std::fs::File;
use std::io::{self, ErrorKind, Read};

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

    // we can use closures to make code cleaner and shorted.
    // it wont contain any match statements
   /* 
    let mut file = File::open(&file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_name).unwrap_or_else(|error| {
                panic!("problem creating the file: {error:?}");
            })
        }
        else {
            panic!("problem opening the file: {error:?}");
        }
    });
    */

    // other functions include unwrap() , which return the Ok if no error and panics otherwise
    // and there is expect() which does the same, but lets us choose the error message. mostly this
    // is used instead of unwrap()



    let mut file_content = String::new();
    let file_read_result = file.read_to_string(&mut file_content);
    
    match file_read_result {
        Ok(_) => println!("{file_content}"),
        Err(error) => println!("error occured while reading file: {error:?}"),
    };


    let username = read_username_from_file(&file_name).unwrap_or_else(|error| {
        panic!("failed to read username: {error:?}");
    });

    println!("username: {username}");
}

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut username_file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
