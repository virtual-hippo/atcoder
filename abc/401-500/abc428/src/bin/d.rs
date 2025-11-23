use num_integer::sqrt;
use proconio::{fastout, input};

fn g(c: u64, r: u64) -> u64 {
    let mut res = 0;
    let mut l = 1;

    loop {
        let base = c * 10 * l;
        let nl = base + l;
        let nr = (base + (l * 10 - 1)).min(base + r);
        if nr < nl {
            return res;
        }
        res += sqrt(nr) - sqrt(nl - 1);

        l *= 10;
    }
}

fn solve() {
    input! {
        c: u64,
        d: u64,
    }

    let ans = g(c, c + d) - g(c, c);
    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(|_| solve());
}
