use std::fs;
use std::io::Error;

fn read_file(file_path: &str) -> Result<String, Error> {
    fs::read_to_string(file_path)
}

fn main() {
    let file_path = "test_file";
    
    let contents = read_file(&file_path).expect(&format!("unable to read file <{}>", file_path));
    println!("{}", contents);
}
