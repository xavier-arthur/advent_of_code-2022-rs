mod util;

use advent::read_stdin;
use util::get_array;
use util::pair_incoporates;
use std::ops::RangeInclusive;

fn main() {

    let stdin = read_stdin();
    let stdin_by_line: Vec<&str> = stdin.iter().map(|v| v.as_str()).collect();
    let mut pairs: Vec<Vec<&str>> = Vec::new();

    for line in stdin_by_line {
        pairs.push(
            line.split(',').collect::<Vec<&str>>()
        );
    }

    let range: Vec<Vec<RangeInclusive<i32>>> = pairs.iter().map(get_array).collect();

    let mut sum = 0;
    for things in range {
        if pair_incoporates(&things) {
            sum += 1;
        }
    }
    println!("{}", sum);
}