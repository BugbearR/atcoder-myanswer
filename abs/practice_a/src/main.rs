use proconio::input;

fn main() {
    input! [
        a: i32,
        b: i32,
        c: i32,
        s: String
    ];
    // println!("a:{}", a);
    // println!("b:{}", b);
    // println!("c:{}", c);
    // println!("s:{}", s);
    println!("{} {}", a + b + c, s);
}
