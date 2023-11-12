use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    let wanted_string = "a";

    let file_path = "file_with_lines";
    let file: File = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        if unwrapped.contains(wanted_string) {
            println!("{}", unwrapped);
        }
    }
}
