use std::process::ExitCode;

fn main() -> ExitCode {

    let mut input = String::new();
    let mut lines = std::io::stdin().lines();

    while let Some(line) = lines.next() {
        let current = line.unwrap();

        input.push_str(&current);
        input.push_str("\n");
    }

    let input_vec: Vec<&str> = input.split("\n").collect();
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

    match input_sum.iter().max() {
        Some(max) => println!("{}", max),
        None => println!("input is empty")
    };

    ExitCode::SUCCESS
}
