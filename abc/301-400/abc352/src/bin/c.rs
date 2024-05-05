use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut max_head = 0;
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        ans += a;
        max_head = max_head.max(b-a);
    }
    println!("{}", ans + max_head);
}

