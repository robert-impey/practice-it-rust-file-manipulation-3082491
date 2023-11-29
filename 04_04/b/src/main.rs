use std::collections::HashMap;

fn get_words(text: &str) -> Vec<String> {
    text.split_whitespace().map(|w|w.to_string()).collect()
}

fn replace_x_with_y_in_place(words: Vec<String>, replacement_map: &HashMap<String, String>) -> Vec<String> {
    let mut updated_words: Vec<String> =  Vec::new();

    for word in words {
        let replacement = replacement_map.get(&word);

        let updated_word = match replacement {
            Some(rep) => rep.to_string(),
            None => word,
        };

        updated_words.push(updated_word);
    }

    updated_words
}

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
    let replacement_map = HashMap::from([
        ("first".to_string(), "last".to_string()),
        ("line".to_string(), "entry".to_string()),
    ]);

    let words = get_words(&contents);
    let new_words = replace_x_with_y_in_place(words.clone(), &replacement_map);

    println!("{:?}", new_words);
}
