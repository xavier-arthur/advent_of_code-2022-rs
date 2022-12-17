mod rock_paper;
mod game;

use advent::read_stdin;
use rock_paper::RockPaper;

fn main() {
    let stdin = read_stdin();
    let stdin_by_line: Vec<&str> = stdin.iter().map(|v| v.as_str()).collect();

    let mut game = RockPaper::default();

    let rigged = match std::env::args().nth(1) {
        Some(v) if v == "rigged" => true,
        _ => false
    };

    for line in stdin_by_line {
        let pair: Vec<&str> = line.split_whitespace().collect();

        let theirs = pair[0];
        let mine   = pair[1];

        game.new(
            mine,
            theirs
        );

        if rigged {
            game.play_rigged();
        } else {
            game.play();
        }
    }

    println!("{}", game.game_score);
}