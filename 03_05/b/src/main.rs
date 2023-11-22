fn get_words(string: &String) -> Vec<String> {
    let mut all_words: Vec<String> = Vec::new();

    let lines: Vec<String> = string.split("\n")
        .map(|s|s.to_string())
        .collect();

    for line in lines {
        for word in line.split(" ") {
            all_words.push(word.to_string());
        }
    }

    all_words
}

fn get_words_fm(string: &String) -> Vec<String> {
    let words: Vec<String> = string.split("\n")
        .flat_map(|l|l.split(" "))
        .map(|w|w.to_string())
        .collect();

    words
}

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");

    let words = get_words(&contents);

    println!("{:?}", words);

    let words_fm = get_words_fm(&contents);

    println!("{:?}", words_fm);
}
