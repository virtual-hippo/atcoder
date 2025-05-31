use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        t: [usize; n],
    }
    let t = std::iter::once(0).chain(t).collect::<Vec<_>>();

    for i in 1..n + 1 {
        let d = t[i] - t[i - 1];
        if d > s {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
