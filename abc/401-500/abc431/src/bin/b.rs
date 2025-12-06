use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        x: usize,
        n: usize,
        w: [usize; n],
        q: usize,
    }

    let mut now = x;
    let mut f = vec![false; n];

    for _ in 0..q {
        input! {
            p: Usize1,
        }
        f[p] = !f[p];
        if f[p] {
            now += w[p]
        } else {
            now -= w[p]
        }

        println!("{}", now);
    }
}
