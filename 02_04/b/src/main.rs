use std::fs;

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("unable to open file")
}

fn main() {
    let file_path = "test_file";

    let contents = read_file(file_path);
    println!("{}", contents);
}
