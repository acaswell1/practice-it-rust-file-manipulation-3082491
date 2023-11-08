use std::fs::read_to_string;

fn main() {
    let file_path = "file_with_lines";

    // Implement the read_file function
    let lines = read_file(file_path).expect(&format!("Unable to read file <{}>", &file_path));

    println!("{:?}", lines);
}

fn read_file(file_path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    Ok( 
        read_to_string(file_path)
            .expect("Unable to read file")
            .split('\n')
            .map(|line| line
                .to_string()
                .split_whitespace()
                .map(|word| word.to_string())
                .collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>()
    )
}