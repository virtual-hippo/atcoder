use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let ans = a.iter().filter(|&x| x % 2 == 1).count();
    println!("{}", ans);
}
