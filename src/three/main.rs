use advent::read_stdin;
use util::{get_duplicates};

mod util;

fn main() {
    let stdin = read_stdin();
    let stdin_by_line: Vec<&str> = stdin.iter().map(|v| v.as_str()).collect();

    let mut stdin_by_char: Vec<Vec<char>> = Vec::new();

    for line in stdin_by_line {
        stdin_by_char.push(line.chars().collect());
    }

    let mut total: i32 = 0;
    for mut entry in stdin_by_char {
        total += get_duplicates(&mut entry);
    }

    println!("{}", total);
}