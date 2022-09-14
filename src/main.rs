use ::regex::Regex;

fn main() {
    println!("Hello, world!");
    assert!(Regex::new("^.+$").unwrap().is_match("a"));
}
