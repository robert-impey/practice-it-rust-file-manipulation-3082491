use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, BufRead};
use std::fs::File;
use std::io::prelude::*;

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

fn main() {
    let wanted_string = "alice";

    let file_path = "alice_chapter_1";
    let file: File = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut alice_line_count = 0;
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        lines.push(unwrapped.clone());

        if unwrapped.to_lowercase().contains(wanted_string) {
            alice_line_count += 1;
        }
    }

    println!("{} appears on {} lines", wanted_string, alice_line_count);

    let contents: String = fs::read_to_string(file_path).expect("unable to read file");

    let words = get_words(&contents);
    let counter = word_count(&words);

    println!("{:#?}", counter);

    println!("The wanted string {} occurs {} times", wanted_string, counter.get(wanted_string).unwrap());

    let mut counter_b: BTreeMap<u32, Vec<String>> = BTreeMap::new();

    let mut word_count_b = 0;

    for (k,v) in counter {
        let words_of_count = counter_b.get(&v);

        let mut words = match words_of_count {
            Some(ws) => ws.to_vec(),
            None => vec![],
        };

        words.push(k);

        word_count_b += v;
        counter_b.insert(v, words);
    }

    println!("Word count from grouping phase {}", word_count_b);

    for (k,v) in counter_b {
        println!("The {} word(s) that occur(s) {} times: {}",  v.len(), k, v.join(", "));
    }

    let replacement_map = HashMap::from([
        ("Alice".to_string(), "Robert".to_string()),
        ("(Alice".to_string(), "(Robert".to_string()),
        ("Alice;".to_string(), "Robert;".to_string()),
    ]);

    let updated_file_path = "robert_chapter_1";

    let mut writer = File::create(updated_file_path).unwrap();

    for line in lines {
        let new_words = replace_x_with_y_in_place(
            line.split_whitespace().map(|w| w.to_string()).collect(),
            &replacement_map);

        println!("{:?}", new_words);

        let all_words = new_words.join(" ");

        writer.write_all(all_words.as_bytes()).unwrap();

        writer.write_all("\n".as_bytes()).unwrap();
    }
}
