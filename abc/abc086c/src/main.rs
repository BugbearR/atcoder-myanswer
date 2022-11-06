fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Input error.");
    s
}

fn main() {
    let l1 = read_line();
    let mut ls1 = l1.split_whitespace();
    let n = ls1.next().unwrap().parse().unwrap();

    let mut cur_t: i32 = 0;
    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;

    for _i in 0..n {
        let l2 = read_line();
        let mut ls2 = l2.split_whitespace();
        let t: i32 = ls2.next().unwrap().parse().unwrap();
        let x: i32 = ls2.next().unwrap().parse().unwrap();
        let y: i32 = ls2.next().unwrap().parse().unwrap();

        let td = t - cur_t;
        let d = (cur_x - x).abs() + (cur_y - y).abs();
    
        if td < d {
            println!("No");
            return;
        }

        if (td & 1) != (d & 1) {
            println!("No");
            return;
        }

        cur_t = t;
        cur_x = x;
        cur_y = y;
    }

    println!("Yes");
}
