use std::fs;

fn write_words_to_file(path: &str, words: &Vec<String>) -> Result<(), std::io::Error> {
    let all_words = words.join(" ");

    fs::write(path, all_words)?;
    Ok(())
}

fn main() {
    let file_path = "words_to_file";
    let words = vec![
        "Words".to_string(),
        "of".to_string(),
        "the".to_string(),
        "first".to_string(),
        "line".to_string(),
    ];

    // Implement the write_words_to_file function
    write_words_to_file(&file_path, &words).unwrap();
}
