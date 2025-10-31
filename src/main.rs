use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

#[allow(warnings)]
fn main() {
    let start = Instant::now();
    let file =
        File::open("C:/Users/yegla/OneDrive/Desktop/all folders/Rust/FileCompresor/dummy.txt")
            .expect("Failed to open the file");
    let reader = BufReader::new(file);
    let MAX_DISTANCE = 128;
    let mut word_buffer: Vec<u8> = Vec::new();
    let mut wordMap: HashMap<String, i32> = HashMap::new();

    for (distance, letter) in reader.bytes().enumerate() {
        let letter = letter.unwrap();
        if letter.is_ascii_alphabetic() {
            word_buffer.push(letter);
        } else if !word_buffer.is_empty() {
            let word = String::from_utf8(word_buffer.clone()).unwrap();
            if word.len() < 5 {
                print!("{}", word);
                print!("{}", String::from_utf8(vec![letter]).unwrap_or_else(|_| String::new()));
                continue;
            }

            let mut distanceBetweenWords = 0;
            if let Some(&map_word) = wordMap.get(&word) {
                distanceBetweenWords = findDistance(distance as i32, &word, map_word);
                print!("{}", makePos(distanceBetweenWords as u8, &word.len()));
            } else {
                print!("{}", word)
            }
            wordMap.insert(
                word.clone(),
                ((distance - word.len()) as usize).try_into().unwrap(),
            );

            word_buffer.clear();
            print!("{}", String::from_utf8(vec![letter]).unwrap_or_else(|_| String::new()))
        }
    }

    println!("Ran in {:.2} ms", start.elapsed().as_millis());
}
#[allow(warnings)]

fn makePos(offset: u8, length: &usize) -> String {
    let mat = format!("@{offset}!{length}");
    mat
}

#[allow(warnings)]
fn write(text: String) -> usize {
    print!("{text} ");
    text.len()
}

#[allow(warnings)]
fn findDistance(distance: i32, word: &str, map_word: i32) -> i32 {
    (distance - 1 - word.len() as i32) - map_word
}
