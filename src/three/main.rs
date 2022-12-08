use util::{get_duplicates};

mod util;

fn main() {
    let stdin_by_line: Vec<&str>;
    let mut stdin = std::io::stdin().lines();
    let mut piped_content = String::new();

    while let Some(line) = stdin.next() {
        let str = line.unwrap();
        piped_content.push_str(&str);
        piped_content.push_str("\n");
    }

    piped_content = piped_content.trim().to_string();
    stdin_by_line = piped_content.split("\n").collect();

    stdin_by_line.len();
    
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