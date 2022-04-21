use std::fs::File;
use std::io::ErrorKind;
use std::io;

pub fn log_error() {
    println!("*******************Chapter 9***********************");
}

pub fn propagate_error() -> Result<String, io::Error> {
    // match File::open("hello.txt") {
    //     Ok(_) => Ok(String::from("file ok")),
    //     Err(e) => Err(e),
    // }
    File::open("hello.txt")?; // equales to match above
    // we’re only allowed to use the ? operator in a function that returns 
    // Result or Option or another type that implements std::ops::Try

    Ok(String::from("file ok"))
}

pub fn error_handler() {
    let f = propagate_error();

    match f {
        Ok(ok_str) => println!("file {} exists", ok_str),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("file not found!"),
            other_error => println!("other error {:?}", other_error),
        },
    };


}