use proconio::input;

fn main() {
    input![
        n: usize,
        a: [u32; n]
    ];

    // for an in a.iter() {
    //     println!("{}", an);
    //     println!("{:b}", an);
    // }

    let mut b: u32 = 1;
    for i in 0..31 {
        if a.iter().any(|x| { /* println!("{:b} {:b}", x, b) ; */ x & b != 0 }) {
            println!("{}", i);
            return;
        }
        b = (b << 1) + 1;
    }
    assert!(true);
}
