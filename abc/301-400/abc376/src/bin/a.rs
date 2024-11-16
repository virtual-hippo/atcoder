use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        t: [usize; n],
    }
    let mut last = 0;
    let mut ans = 0;
    for i in 0..n {
        if i == 0 {
            ans += 1;
            last = t[i];
        } else {
            if t[i] - last >= c {
                ans += 1;
                last = t[i];
            }
        }
    }
    println!("{}", ans);
}
