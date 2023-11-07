use std::fs::read_to_string;

fn main() {
    let wanted_string = "a";
    read_to_string("file_with_lines")
        .expect("unable to open file")
        .rsplit('\n')
        .rev()
        .filter(|line| line.contains(wanted_string))
        .for_each(|line| println!("{}", line));
}
