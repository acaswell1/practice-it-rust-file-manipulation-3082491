use std::fs::read_to_string;

fn main() {
    let file_path = "file_with_lines";
    let wanted_string = "a";

    // Create the below function
    //print_wanted_lines_from_file(file_path, wanted_string);
    print_wanted_lines_from_file(file_path, wanted_string);
}

fn print_wanted_lines_from_file(file_path: &str, wanted_string: &str) {
    read_to_string(file_path)
        .expect("unable to open file")
        .split('\n')
        .filter(|line| line.contains(wanted_string))
        .for_each(|line| println!("{}", line));
}
