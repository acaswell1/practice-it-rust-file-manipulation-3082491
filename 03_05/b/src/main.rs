fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");

    // Implement the get_words function
    let words = get_words(&contents);

    println!("{:?}", words);
}

fn get_words(contents: &str) -> Vec<String> {
    contents
    .split_whitespace()
    .map(|word| word.to_string())
    .collect()
}
