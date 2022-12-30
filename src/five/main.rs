mod queue;
mod util;

use advent::{read_stdin};
use util::Directive;

fn parse_crates(crates: Vec<String>) -> Vec<Vec<char>> {
    let column_count = crates.iter().fold(0, |carry, current| {
        let count =  current.matches("]").count();

        if count > carry {
            count
        } else {
            carry
        }
    });

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); column_count];

    for line in crates {
        for (i, v) in line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .into_iter()
            .enumerate() {

            if v[1].is_alphabetic() {
                stacks[i].push(v[1]);
            }
        }
    }

    stacks
}

fn parse_directives(directives: Vec<String>) -> Vec<Directive> {
    let mut dir : Vec<Directive> = Vec::new();

    for v in directives {
        let dir_by_word: Vec<&str> = v.split(" ").collect();

        let target = dir_by_word[1].parse::<u32>().unwrap();
        let from   = dir_by_word[3].parse::<usize>().unwrap() - 1;
        let to     = dir_by_word[5].parse::<usize>().unwrap() - 1;

        dir.push(Directive::new(from, to, target));
    }

    dir
}

fn process_directive(stacks: &mut Vec<Vec<char>>, dir: Directive) {
    let mut tmp: Vec<char> = vec![];

    for _ in 0..dir.quantity {
        tmp.push(
            *stacks[dir.from].first().unwrap()
        );
        stacks[dir.from].remove(0);
    }

    for v in tmp {
        stacks[dir.to].insert(0, v);
    }
}

fn main() {
    let input  = read_stdin();
    let crates = input[0..8].to_vec();
    let directives = input[10..].to_vec();
    drop(input);

    let mut stacks = parse_crates(crates);
    let commands = parse_directives(directives);

    for v in commands {
        process_directive(&mut stacks, v);
    }

    for ch in stacks {
        print!("{}", ch[0]);
    }

    println!("");
}
