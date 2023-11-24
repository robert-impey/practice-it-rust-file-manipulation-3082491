use std::fs;

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let content = fs::read_to_string(path)?;

    let lines: Vec<String> = content
        .lines()
        .map(|l|l.to_string())
        .collect();

    let words: Vec<Vec<String>> = lines
        .iter()
        .map(|l|l.split(" ").map(|w|w.to_string()).collect())
        .collect();

    Ok(words)
}

fn main() {
    let file_path = "file_with_lines";

    let lines = read_file(&file_path).expect(&format!("Unable to read file <{}>", &file_path));

    println!("{:?}", lines);
}
