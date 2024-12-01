use std::fs;

pub fn read_file(filepath: &str) -> Vec<String> {
    let contents = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");

    contents.split("\n").map(|s| s.to_string()).collect()
}