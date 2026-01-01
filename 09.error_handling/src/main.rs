use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening file: {err:?}");
            }
        },
    };

    // unwrap return value inside the OK,
    // if the Result is an Error, unwrap will call the panic!
    let _ = File::open("test_1.txt").unwrap();

    // expect is the same as unwrap but you can custom the error message
    let _ = File::open("test_2.txt").expect("test_2.txt should be included in this project");

    let username = match read_username_from_file() {
        Ok(username) => username,
        Err(err) => panic!("Failed to process file: {err:?}"),
    };
    println!("username: {username}");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();

    // ? operator work in almost same way as the match expressions
    // if value of the Result is an Ok, the value inside Ok get returned
    // if value is an Err, the Err will be returned
    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
