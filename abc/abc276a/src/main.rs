use proconio::input;

fn main() {
    input![
        s: String
    ];

    match s.rfind('a') {
        Some(result) => println!("{}", result + 1),
        None => println!("-1")
    }
}
