use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // panic!("Dieee")

    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("There was a problem opening the file: {:?}", error),
    // };

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        // ? means if there is an error, propagate it.
        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
}
