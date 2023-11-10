use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let file_path = "file_with_lines";
    let file: File = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
