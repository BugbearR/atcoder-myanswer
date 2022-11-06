use proconio::input;

fn combine(a: i32, b: i32, c: i32, x: i32) -> i32 {
    println!("{} {} {} {}", a, b, c, x);
    if x == 0 {
//        println!("1");
        return 1
    }

    if x == 1 && c > 0 {
//        println!("1");
        return 1
    }
    if x == 2 && c == 0 && b > 0 {
//        println!("1");
        return 1
    }
    if x == 10 && c == 0 && b == 0 && a > 0 {
//        println!("1");
        return 1
    }

    let mut s = 0;
    if x >= 10 && a > 0 {
        s += combine(a - 1, b, c, x - 10)
    }
    if x >= 2 && b > 0 {
        s += combine(a, b - 1, c, x - 2)
    }
    if x >= 1 && c > 0 {
        s += combine(a, b, c - 1, x - 1)
    }

//    println!("{}", s);
    s
}

fn main() {
    input![
        a: i32,
        b: i32,
        c: i32,
        x: i32
    ];

    let r = combine(a, b, c, xd);
    println!("{}", r);
}
