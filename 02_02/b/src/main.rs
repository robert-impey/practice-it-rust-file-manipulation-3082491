use std::fs;

fn main() {
    let file_path = "file_with_lines";
    let wanted_string = "a";

    print_wanted_lines_from_file(file_path, wanted_string);
}

fn print_wanted_lines_from_file(file_path: &str, wanted_string: &str) {
    let contents: String = fs::read_to_string(file_path).expect("unable to open file");

    for line in contents.lines() {
        if line.contains(wanted_string) {
            println!("{}", line);
        }
    }
}