//legacy
fn print_result(res: Vec<String>) {

}

use std::io;
use std::io::Write;

pub fn parse_tags_input() -> Vec<String> {
    let mut line = String::new();
    print!("enter tags:");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    let res = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    io::stdout().flush();
    res
}