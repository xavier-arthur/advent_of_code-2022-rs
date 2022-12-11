pub fn read_stdin() -> Vec<String> {
    let mut stdin = std::io::stdin().lines();
    let mut piped_content = String::new();
    let stdin_by_line: Vec<String>;

    while let Some(v) = stdin.next() {
        let str = v.unwrap();
        piped_content.push_str(&str);
        piped_content.push_str("\n");
    }

    stdin_by_line = piped_content.trim().split("\n").map(String::from).collect();

    stdin_by_line
}