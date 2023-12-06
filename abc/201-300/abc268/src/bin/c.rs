use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut c = vec![0; n];
    for i in 0..n {
        c[(n + p[i] - i - 1) % n] += 1;
        c[(n + p[i] - i) % n] += 1;
        c[(n + p[i] - i + 1) % n] += 1;
    }
    let ans = c.iter().max().unwrap();
    println!("{}", ans);
}
