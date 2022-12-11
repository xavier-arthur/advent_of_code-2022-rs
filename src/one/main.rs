use std::{process::ExitCode, env::args};

use advent::read_stdin;

fn main() -> ExitCode {

    /* 
        if the first argument is "top", then the program will print the sum of the top three
    */

    let top_three = match args().nth(1) {
        Some(v) if v == "top" => true,
        _  => false
    };

    let stdin = read_stdin();
    let input_vec: Vec<&str> = stdin.iter().map(|v| v.as_str()).collect();

    let mut input_sum: Vec<u32> = Vec::new();
    let mut current_int: u32 = 0;

    for line in input_vec {
        if line.len() == 0 {
            input_sum.push(current_int);
            current_int = 0;
            continue;
        }
        current_int += line.parse::<u32>().unwrap();
    }

    if top_three {
        input_sum.sort();
        input_sum.reverse();

        let mut sum: u32 = 0;
        for i in 0..3 {
            sum += input_sum[i];
        }

        println!("{}", sum);
    } else {
        match input_sum.iter().max() {
            Some(max) => println!("{}", max),
            None => println!("input is empty")
        };
    }

    ExitCode::SUCCESS
}
