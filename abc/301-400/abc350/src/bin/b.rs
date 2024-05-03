use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }
    let mut ha = vec![true; n];
    for i in 0..q {
        ha[t[i] - 1] = !ha[t[i] - 1];
    }
    let ans = ha.iter().filter(|&&v| v).count();
    println!("{}", ans);
}
