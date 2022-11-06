use proconio::input;
use regex::Regex;

fn main() {
    input![
        s: String
    ];

    let re = Regex::new(r"^(dream|dreamer|erase|eraser)*$").unwrap();
    println!("{}", if re.is_match(&s) { "YES" } else { "NO" });
}
