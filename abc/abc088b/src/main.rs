use proconio::input;

fn solve(_n: i32, a: Vec::<i32>) -> i32 {
    let mut aw = a.clone();
    aw.sort_unstable_by(|a, b| b.cmp(a));
    let alice: _ = aw.iter().step_by(2).sum::<i32>();
    let bob: _ = aw.iter().skip(1).step_by(2).sum::<i32>();

    alice - bob    
}

fn main() {
    input![
        n: i32,
        a: [i32; n]
    ];

    let r = solve(n, a);
    println!("{}", r);
}
