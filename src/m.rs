use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[allow(non_snake_case)]
fn main() {
    let start = Instant::now();
    let file =
        File::open("C:/Users/yegla/OneDrive/Desktop/all folders/Rust/FileCompresor/dummy.txt")
            .expect("Failed to open the file");
    let reader = BufReader::new(file);
    let mut wordCounter: HashMap<String, u64> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        line.split(|c: char| !c.is_alphabetic()).for_each(|word| {
            wordCounter
                .entry(word.to_string())
                .and_modify(|entry| {
                    *entry += 1;
                })
                .or_insert(1);
        });
    }
    wordCounter
        .iter()
        .filter(|(word, _)| word.len() > 6)
        .for_each(|(word, count)| println!("{}:{}", word, count));

    println!("Ran in {:.2} ms", start.elapsed().as_millis());
}
