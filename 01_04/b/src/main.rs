use std::fs::read_to_string;

fn main() {
  read_to_string("file_with_lines")
    .unwrap_or_default()
    .rsplit('\n')
    .rev()
    .for_each(|line| println!("{}", line));
}
