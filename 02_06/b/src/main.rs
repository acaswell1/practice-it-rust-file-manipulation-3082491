use std::fs::read_to_string;

fn main() {
    let file_path = "test_file";
    
    // Implement the read_file function
    let contents = read_file(file_path).unwrap_or_else(|_| panic!("unable to read file <{}>", file_path));
    println!("{}", contents);
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = read_to_string(file_path)?;
    Ok(content)
}
