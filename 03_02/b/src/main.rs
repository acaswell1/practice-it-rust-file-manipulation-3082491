use std::fs::read_to_string;

fn main() {
    let file_path = "file_with_lines";

    // Implement the read_file function
    let lines: Vec<String> = read_file(file_path).unwrap_or_else(|_| panic!("Error reading file <{}>", &file_path));

    println!("{:#?}", lines);
}

fn read_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = read_to_string(file_path)?;
    Ok(
        content
        .split('\n')
        .map(|line| line.to_string())
        .collect()
    )
}
