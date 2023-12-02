use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    }
    let mut ans = s.min(m).min(l) * n;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if (i * 6) + (j * 8) + (k * 12) >= n {
                    ans = ans.min(i * s + j * m + k * l);
                }
            }
        }
    }

    println!("{}", ans);
}
