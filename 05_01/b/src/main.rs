use std::fs::{read_to_string, write};

fn main() {
    let mut replaced_text: Vec<Vec<String>> = vec![vec![]];

    match read_file("alice_chapter_1") {
        Ok(ch_1) => {
          let formatted_text = format_text(&ch_1);
          for line in &formatted_text {
            replaced_text
              .push(replace_x_with_y_in_place(line.clone(), "Alice"));
          }
          write_file("replaced_with_alec", &stringify(replaced_text))

        },
        Err(_) => panic!("Error opening file"),
    };

}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = read_to_string(file_path)?;
    Ok(content)
}

fn write_file(file_path: &str, content: &str) {
    let _ = write(file_path, content);
}

fn format_text(content: &str) -> Vec<Vec<String>> {
    let whole_lines: Vec<String> = content.split('\n').map(|line| line.to_string()).collect();

    whole_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn stringify(content: Vec<Vec<String>>) -> String {
    content
        .iter()
        .map(|line| {
            line.iter()
                .rfold("".to_string(), |acc, word| word.to_string() + " " + &acc)
        })
        .collect::<Vec<String>>()
        .iter()
        .fold("".to_string(), |acc, line| acc + line)
}

fn replace_x_with_y_in_place(mut words: Vec<String>, to_replace: &str) -> Vec<String> {
    words.iter_mut().for_each(|word| {
        if word.contains(to_replace) {
            *word = word.replace(to_replace, "Alec");
        }
    });

    words
}
