use std::collections::HashMap;
use std::io::{self};

// inssisting to implement more even though could have less computation direclty
// cuz why not over complicate things and learn some syntax in the meantime
fn ingest_and_count(input: &str) -> HashMap<String, usize> {
    let words: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|i| i.to_string())
        .collect();
    let mut map: HashMap<String, usize> = HashMap::new();
    for i in 0..words.len() {
        *map.entry(words[i].clone()).or_insert(0) += 1;
    }
    return map;
}

fn rank(map: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut words: Vec<(String, usize)> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();

    words.sort_by(|a, b| {
        let primary = b.1.cmp(&a.1);
        if primary == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            primary
        }
    });
    return words;
}

fn test_iac() {
    let test_input = "a b a c a b";
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), 3);
    expected.insert("b".to_string(), 2);
    expected.insert("c".to_string(), 1);
    assert_eq!(ingest_and_count(&test_input), expected);
}

fn test_rank() {
    let input = "b a a c b a";
    let map = ingest_and_count(input);
    let ranked = rank(&map);
    let expected = vec![
        ("a".to_string(), 3),
        ("b".to_string(), 2),
        ("c".to_string(), 1),
    ];
    assert_eq!(ranked, expected);
}

fn main() {
    test_iac();
    test_rank();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let word_map: HashMap<String, usize> = ingest_and_count(&input);
    input.clear();
    let word_vec: Vec<(String, usize)> = rank(&word_map);

    // top 3 with for loop
    // let length = if word_vec.len() <3 {word_vec.len()} else {3};
    // for i in 0..length {
    //     let index = i + 1;
    //     println!("{}. {} ({})", index, word_vec[i].0,word_vec[i].1);
    // }

    // top 3 with iterators
    for (i, (word, count)) in word_vec.iter().take(3).enumerate() {
        println!("{}. {} ({})", i, word, count);
    }
}
