use ac_library::FenwickTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let s = {
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + a[i];
        }
        for i in 1..n + 1 {
            s[i] = s[i] % m;
        }
        s
    };
    let v1: usize = (1..n + 1).map(|i| (s[i] * i)).sum();
    let v2: usize = (0..n).map(|i| (s[i] * (n - i))).sum();

    let v3 = {
        let mut bit = FenwickTree::new(m, 0);
        let mut ret = 0;

        for i in 0..n + 1 {
            if (s[i] + 1) <= m {
                ret += bit.sum((s[i] + 1)..m);
            }
            bit.add(s[i], 1);
        }
        ret * m
    };

    let ans = v1 + v3 - v2;
    println!("{}", ans);
}
