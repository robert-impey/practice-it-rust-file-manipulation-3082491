use std::collections::HashMap;

fn get_words(text: &String) -> Vec<String> {
    text.split_whitespace().map(|w| w.to_string()).collect()
}

fn word_count(words: &Vec<String>) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();

    for word in words {
        let word_lc = word.to_lowercase();
        let count = counts.get(&*word_lc);

        let updated_count = match count {
            Some(cnt) => cnt +1,
            None => 1,
        };

        counts.insert(word_lc, updated_count);
    }

    counts
}

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");

    let words = get_words(&contents);
    let counter = word_count(&words);

    println!("{:#?}", counter);
}
