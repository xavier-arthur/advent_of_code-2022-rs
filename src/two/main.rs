mod rock_paper;
mod game;

use rock_paper::RockPaper;

fn main() {
    let stdin_by_line: Vec<&str>;

    let mut pipe_content = String::new();
    let mut lines = std::io::stdin().lines();

    while let Some(v) = lines.next() {
        let str = v.unwrap();

        pipe_content.push_str(&str);
        pipe_content.push_str("\n");
    }
    pipe_content = pipe_content.trim().to_string();
    stdin_by_line = pipe_content.split("\n").collect();

    let mut game = RockPaper::default();

    for line in stdin_by_line {
        let pair: Vec<&str> = line.split_whitespace().collect();

        let theirs = pair[0];
        let mine   = pair[1];

        game.new(
            mine,
            theirs
        );

        game.play();
    }

    println!("{}", game.game_score);
}