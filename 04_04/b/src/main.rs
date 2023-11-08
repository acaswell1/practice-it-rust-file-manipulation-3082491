use std::collections::HashMap;

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
    let replacement_map = HashMap::from([
        ("first".to_string(), "last".to_string()),
        ("line".to_string(), "entry".to_string()),
    ]);

    // Implement the get_words and replace_x_with_y_in_place functions
    let words = get_words(&contents);
    let new_words = replace_x_with_y_in_place(words.clone(), &replacement_map);

    println!("{:?}", new_words);
}

fn get_words(content: &str) -> Vec<String> {
    content
        .split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

fn replace_x_with_y_in_place(mut words: Vec<String>, change_map: &HashMap<String, String>) -> Vec<String>{
    words
        .iter_mut()
        .for_each(|word | match change_map.get(word) {
            Some(change_word) => *word = change_word.to_string(),
            _ => *word = word.to_string()
        });
    
    words
}
