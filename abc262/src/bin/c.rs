use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut koho1 = 0;
    let mut set = HashSet::new();
    for i in 0..n {
        if a[i] == i + 1 {
            koho1 += 1;
        } else {
            let j = a[i] - 1;
            if std::cmp::min(a[i], a[j]) == i + 1 &&
            std::cmp::max(a[i], a[j]) == j + 1 && i < j {
                set.insert((i, j));
            }
        }
    }
    let mut ans = 0;
    ans += (koho1 * (koho1 -1)) / 2;
    ans += set.len();
    println!("{}", ans);
}

