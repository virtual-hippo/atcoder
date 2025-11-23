use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }
    a.sort();

    // 累積和
    let s = std::iter::once(0_usize)
        .chain(a.iter().scan(0_usize, |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .collect::<Vec<_>>();

    for _ in 0..q {
        input! {
            b: usize,
        }
        if b == 1 {
            println!("{}", 1);
            continue;
        }

        let xi = a.lower_bound(&b);

        if xi == n {
            println!("-1");
            continue;
        }

        let v = (b - 1) * (n - xi) + 1;

        let ans = s[xi] + v;
        if ans <= s[n] {
            println!("{}", ans);
        } else {
            println!("{}", -1);
        }
    }
}
