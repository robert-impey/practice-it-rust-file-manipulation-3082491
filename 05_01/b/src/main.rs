use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, BufRead};
use std::fs::File;

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

fn write_words_to_file(path: &str, words: &Vec<String>) -> Result<(), io::Error> {
    let all_words = words.join(" ");

    fs::write(path, all_words)?;
    Ok(())
}

fn main() {
    let wanted_string = "alice";

    let file_path = "alice_chapter_1";
    let file: File = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut alice_line_count = 0;

    for line in reader.lines() {
        let unwrapped = line.unwrap().to_lowercase();
        if unwrapped.contains(wanted_string) {
            alice_line_count += 1;
        }
    }

    println!("{} appears on {} lines", wanted_string, alice_line_count);

    let contents: String = fs::read_to_string(file_path).expect("unable to read file");

    let words = get_words(&contents);
    let counter = word_count(&words);

    println!("{:#?}", counter);

    println!("The wanted string {} occurs {} times", wanted_string, counter.get(wanted_string).unwrap());

    let mut counter_b : BTreeMap<u32, String> = BTreeMap::new();

    for (k, v) in counter {
        counter_b.insert(v, k);
    }

    for (k, v) in counter_b {
        println!("{} occurs {} times", v, k);
    }

    let replacement_map = HashMap::from([
        ("Alice".to_string(), "Robert".to_string()),
        ("(Alice".to_string(), "(Robert".to_string()),
        ("Alice;".to_string(), "Robert;".to_string()),
    ]);

    let new_words = replace_x_with_y_in_place(words.clone(), &replacement_map);

    println!("{:?}", new_words);

    let updated_file_path = "robert_chapter_1";

    write_words_to_file(&updated_file_path, &new_words).unwrap();
}
