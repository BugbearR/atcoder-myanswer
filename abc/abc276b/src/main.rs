fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Input error.");
    s
}

fn main() {
    let l1 = read_line();
    let mut ls1 = l1.split_whitespace();
    let n = ls1.next().unwrap().parse().unwrap();
    let m = ls1.next().unwrap().parse().unwrap();

    let mut v = vec![vec![0; 0]; n];

    for _i in 0..m {
        let lm = read_line();
        let mut lms = lm.split_whitespace();
        let a: usize = lms.next().unwrap().parse().unwrap();
        let b: usize = lms.next().unwrap().parse().unwrap();

        v[a-1].push(b);
        v[b-1].push(a);
    }

    for i in 0..(n as usize) {
        v[i].sort();
        print!("{}", v[i].len());
        for k in 0..v[i].len() {
            print!(" {}", v[i][k]);
        }
        println!();
    }
}
