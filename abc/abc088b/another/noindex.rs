use proconio::input;

fn solve(n: i32, a: Vec::<i32>) -> i32 {
    let mut aw = a.clone();
    aw.sort_unstable_by(|a, b| b.cmp(a));
    let mut alice = 0;
    let mut bob = 0;
    let mut idx = 0;
    for item in aw {
        if idx & 1 == 0 {
            alice += item;
        }
        else {
            bob += item;
        }
        idx = idx + 1;
    }

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
