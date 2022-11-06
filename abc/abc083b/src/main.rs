use proconio::input;

fn solve(n: i32, a: i32, b: i32) -> i32 {
    (1..(n + 1)).filter(|x| {
        let c0 = x % 10;
        let c1 = (x / 10) % 10;
        let c2 = (x / 100) % 10;
        let c3 = (x / 1000) % 10;
        let c4 = (x / 10000) % 10;
        let c = c0 + c1 + c2 + c3 + c4;
        // assert_eq!(format!("{}{}{}{}", c3, c2, c1, c0).to_string(), format!("{:0>4}", x).to_string());
        // println!("{} {}{}{}{}", x, c3, c2, c1, c0);
        a <= c && c <= b
    }).sum()
}

fn main() {
    input![
        n: i32,
        a: i32,
        b: i32
    ];
    let r = solve(n, a, b);
    println!("{}", r);
}
