use proconio::input;

fn solve(n: usize, p: Vec<i32>) -> Vec<i32> {
    let mut pw = p.clone();
    // 下から順番が逆転しているところを探す
    let hi_pos =
        (0..(n-1))
        .rev()
        .find(|x| pw[*x] > pw[*x+1])
        .expect("Reversal position not found.");

    let hi_val = pw[hi_pos];
    // println!("hi_pos:{} hi_val:{}", hi_pos, hi_val);
    // その桁より後の数で最大の数を探す
    let (low_pos, low_val) =
        pw[hi_pos+1..n]
            .iter()
            .enumerate()
            .fold((0, -1), // (usize::MIN, i32::MIN)
                |(i_a, a), (i_b, b)| {
                    if hi_val > *b && *b > a {
                        (i_b + hi_pos+1, *b)
                    }
                    else {
                        (i_a, a)
                    }
                });

    // println!("low_pos:{} low_val:{}", low_pos, low_val);
    // 交換する
    pw[low_pos] = hi_val;
    pw[hi_pos] = low_val;

    // 下の桁を逆順ソートする。
    pw[hi_pos+1..].sort_unstable_by(|a, b| b.cmp(a));

    pw
}

fn main() {
    input![
        n: usize,
        p: [i32; n]
    ];

    let r = solve(n, p);
    print!("{}", r[0]);
    for i in 1..r.len() {
        print!(" {}", r[i]);
    }
    println!();
}
