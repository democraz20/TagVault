use frontend;

fn main() {
    println!("TagVault");
    let tags = frontend::parse_tags_input();
    dbg!(tags);
}
