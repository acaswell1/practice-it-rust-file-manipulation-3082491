use std::fs;

fn main() {
    let read_text = fs::read("test_file").unwrap();
    println!("{:?}", read_text);
}
