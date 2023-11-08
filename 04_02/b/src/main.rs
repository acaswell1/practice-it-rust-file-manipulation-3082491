use std::collections::HashMap;

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");

    // Implement the get_words and word_cout functions
    let words = get_words(&contents);
    let counter = word_count(&words);

    println!("{:#?}", counter);
}

fn get_words(content: &str) -> Vec<String> {
    content
        .split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

fn word_count(words: &Vec<String>) -> HashMap<&String, i8> {
    let mut counter:HashMap<&String, i8> = HashMap::new();
    
    for word in words {
        counter.entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    
    counter
}
