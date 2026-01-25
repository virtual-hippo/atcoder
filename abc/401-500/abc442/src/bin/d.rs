use ac_library::*;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
    }

    let mut bit = FenwickTree::new(n, 0);
    for i in 0..n {
        bit.add(i, a[i]);
    }

    for _ in 0..q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {
                    i: Usize1,
                }

                bit.add(i, a[i + 1] - a[i]);
                bit.add(i + 1, a[i] - a[i + 1]);

                let tmp = a[i + 1];
                a[i + 1] = a[i];
                a[i] = tmp;
            },
            2 => {
                input! {
                    l: Usize1,
                    r: usize,
                }

                let ans = bit.sum(l..r);
                println!("{}", ans);
            },
            _ => unreachable!(),
        }
    }
}
