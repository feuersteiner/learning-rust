use std::collections::HashMap;
use std::io::{self};

fn ingest_and_count(input: &str) -> HashMap<String, usize> {
    let words: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|i| i.to_string())
        .collect();
    let mut map: HashMap<String, usize> = HashMap::new();
    for i in 0..words.len() {
       *map.entry(words[i].clone()).or_insert(0) +=1;
    }
    return map;
}

// fn rank(map: HashMap<String, usize>) -> Vec<(String, usize)> {}

fn test_iac() {
    let test_input = "a b a c a b";
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), 3);
    expected.insert("b".to_string(), 2);
    expected.insert("c".to_string(), 1);
    assert_eq!(ingest_and_count(&test_input), expected);
}

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let word_map: HashMap<String, usize> = ingest_and_count(&input);
    input.clear();

    test_iac();
}
