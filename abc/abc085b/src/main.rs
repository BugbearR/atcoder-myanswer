use proconio::input;

fn solve(n: i32, d: Vec<i32>) -> i32 {
    let mut dw = d.clone();
    dw.sort_unstable_by(|a, b| b.cmp(a));
    let mut c = 1;
    for i in 1..(n as usize) {
        if dw[i-1] != dw[i] {
            c = c + 1;
        }
    }

    c
}

fn main() {
    input![
        n: i32,
        d: [i32; n]
    ];

    let r = solve(n, d);
    println!("{}", r);
}
