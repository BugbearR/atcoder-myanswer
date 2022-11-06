use proconio::input;

fn solve(n: usize, a: Vec<i32>) -> i32 {
    let mut base = vec![0; n];
    let mut d2a = vec![0; n];
    let mut d3a = vec![0; n];

    for i in 0..n {
        let mut w = a[i];
        let mut d2c = 0;
        while  (w % 2) == 0 {
            w = w / 2;
            d2c = d2c + 1;
        }
        let mut d3c = 0;
        while (w % 3) == 0 {
            w = w / 3;
            d3c = d3c + 1;
        }
        // println!("d2c:{}", d2c);
        // println!("d3c:{}", d3c);
        // println!("base:{}", w);
        d2a[i] = d2c;
        d3a[i] = d3c;
        base[i] = w;
    }
    // 2と3で割り切れなかった数が全部同じでないと無理
    if base.iter().any(|x| *x != base[0]) {
        return -1;
    }

    let d2min = d2a.iter().min().unwrap();
    let d3min = d3a.iter().min().unwrap();
    // println!("d2min:{}", d2min);
    // println!("d3min:{}", d3min);

    let d2sum: i32 = d2a.iter().map(|x| x - d2min).sum();
    let d3sum: i32 = d3a.iter().map(|x| x - d3min).sum();
    // println!("d2sum:{}", d2sum);
    // println!("d3sum:{}", d3sum);
    d2sum + d3sum
}

fn main() {
    input![
        n: usize,
        a: [i32; n]
    ];

    let r = solve(n, a);
    println!("{}", r);
}
