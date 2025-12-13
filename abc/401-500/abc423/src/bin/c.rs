use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: usize,
        l: [usize; n],
    }
    let s = std::iter::once(0)
        .chain(l.iter().scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .collect::<Vec<_>>();

    let mut p = r;
    let mut ans = 0;

    while s[p] - s[0] != p && p > 0 {
        if l[p - 1] == 1 {
            ans += 1;
        }
        p -= 1;
    }

    while p < r {
        ans += 1;
        p += 1;
    }

    while s[n] - s[p] != n - p && p < n {
        if l[p] == 0 {
            ans += 1;
        }
        if l[p] == 1 {
            ans += 2;
        }
        p += 1;
    }
    println!("{}", ans);
}
