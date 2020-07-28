use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let file_content = open_file();
    lex(&file_content);
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

fn lex(file_content: &str) {
    let mut tokens = String::new();
    let mut state: i32 = 0;
    let mut to_print = String::new();

    for c in file_content.chars() {
        tokens.push(c);
        if tokens == " " {
            tokens = "".to_string();
        } else if tokens == "print" {
            println!("Found: \"print\"");
            tokens = "".to_string();
        } else if tokens == "\"" {
            if state == 0 {
                state = 1;
            } else if state == 1 {
                println!("Found a string.");
                to_print = "".to_string();
                state = 0;
            }
        } else if state == 1 {
            to_print.push(c);
            tokens = "".to_string();
        }
    }
}
