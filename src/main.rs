use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("Hello, world!");
}

fn open_file() -> String {
    let first_arg: String = env::args().nth(1).expect("Please enter a valid filename");
    let file_content: String = read_to_string(&first_arg).expect("Failed to read the file");

    file_content
}

fn read_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => Ok(file_content),
        Err(e) => Err(e),
    }
}
