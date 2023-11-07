use std::fs::write;

fn main() {
    let file_path = "words_to_file".to_string();
    let words = vec![
        "Words".to_string(),
        "of".to_string(),
        "the".to_string(),
        "first".to_string(),
        "line".to_string(),
    ];

    // Implement the write_words_to_file function
    write_words_to_file(&file_path, &words).unwrap();
}

fn write_words_to_file(file_path: &str, words: &Vec<String>) -> std::io::Result<()> {
    let mut write_word: String = "Text Written: ".to_string();
    for word in words {
        write_word += &(word.to_owned() + " ");
    }
    write(file_path, write_word)
}
