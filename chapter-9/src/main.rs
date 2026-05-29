use std::fs::File;
use std::error::Error;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
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


    // let username = read_username_from_file(&file_name).unwrap_or_else(|error| {
    //    panic!("failed to read username from file: {error:?}");
    //});

    let username = read_username_from_file_2(&file_name).unwrap_or_else(|error| {
        panic!("failed to read username from file: {error:?}");
    });

    println!("username: {username}");

    let last_character = last_char(&username).unwrap();
    println!("last character: {last_character}");

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

fn read_username_from_file_2 (file_name: &str) -> Result<String, io::Error> {
    let mut username = String::new();


    // we can use ? when theres chance for error. its like using match.
    // when there is no issue, we get the Ok. and if there is any error,
    // it returns the error.
    // ? is only used in functions whose return type is comaptible with value of ?
    // it can also be Option, or Result
    File::open(file_name)?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
