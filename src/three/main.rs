use advent::read_stdin;
use util::{get_duplicates};

use crate::util::Score;

mod util;

fn process_rucksack(input: Vec<&str>) -> i32 {

    let mut stdin_by_char: Vec<Vec<char>> = Vec::new();

    for line in input {
        stdin_by_char.push(line.chars().collect());
    }

    let mut total: i32 = 0;
    for mut entry in stdin_by_char {
        total += get_duplicates(&mut entry);
    }

    total
}

fn process_badge(mut input: Vec<&str>) -> i32 {
    let score = Score::new();

    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut total = 0;

    while input.len() > 0 {
        let three: Vec<&str> = input.splice(0..=2, None).collect();
        groups.push(three);
    }

    'outer:
    for group in groups {
        for char in group[0].chars() {
            if group[1].contains(char) && group[2].contains(char) {
                total += score.get_points(char);
                continue 'outer;
            }
        }
    }

    total
}

fn main() {
    let stdin = read_stdin();
    let stdin_by_line: Vec<&str> = stdin.iter().map(|v| v.as_str()).collect();

    let badge = match std::env::args().nth(1) {
        Some(v) => v == "badge",
        _ => false
    };

    let total: i32 = if badge {
        process_badge(stdin_by_line)
    } else {
        process_rucksack(stdin_by_line)
    };

    println!("{}", total);
}