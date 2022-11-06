use proconio::input;

fn solve(a: i32, b: i32, c: i32, x: i32) -> i32 {
    let mut s = 0;
    for na in 0..(a + 1) {
        let w = x - 500 * na;
        // println!("{} {}", na, w);
        if w < 0 {
            break;
        }
        else if w == 0 {
            // println!("ok");
            s = s + 1;
        }
        else {
            for nb in 0..(b + 1) {
                let w2 = w - 100 * nb;
                // println!("{} {} {}", na, nb, w);
                if w2 < 0 {
                    break;
                }
                else if w2 == 0 {
                    // println!("ok");
                    s = s + 1;
                }
                else {
                    // println!("{} {} {} {}", na, nb, w, c * 50);
                    if w2 <= c * 50 {
                        // println!("ok");
                        s = s + 1;
                    }
                }
            }
        }
    }

    s
}

fn main() {
    input![
        a: i32,
        b: i32,
        c: i32,
        x: i32
    ];

    let r = solve(a, b, c, x);
    println!("{}", r);
}
