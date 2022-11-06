use proconio::input;

fn solve(n: i32, y: i32) -> [i32; 3] {
    for m10 in 0..(n + 1) {
        let n1 = n - m10;
        let y1 = y - 10000 * m10;

        if n1 == 0 && y1 == 0 {
            return [m10, 0, 0];
        }
        if n1 < 0 || y1 < 0 {
            break;
        }

        for m5 in 0..(n1 + 1) {
            let n2 = n1 - m5;
            let y2 = y1 - 5000 * m5;
            if n2 < 0 || y2 < 0 {
                break;
            }

            // if n2 == 0 && y2 == 0 {
            //     return [m10, m5, 0];
            // }

            if n2 == y2 / 1000 {
                return [m10, m5, n2];
            }
        }
    }
    return [-1, -1, -1];
}

fn main() {
    input![
        n: i32,
        y: i32
    ];

    let r = solve(n, y);
    println!("{} {} {}", r[0], r[1], r[2]);
}
