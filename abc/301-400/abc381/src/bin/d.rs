use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    for si in 0..2 {
        let mut r = si;
        let mut set = HashSet::new();
        for l in si..n {
            if l % 2 != si % 2 {
                continue;
            }
            r = r.max(l);

            while r + 1 < n && a[r] == a[r + 1] && !set.contains(&a[r]) {
                set.insert(a[r]);
                r += 2;
            }
            set.remove(&a[l]);
            if r > l {
                ans = ans.max(r - l);
            }
        }
    }

    println!("{}", ans);
}
