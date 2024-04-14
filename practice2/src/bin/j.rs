use ac_library::{Max, Segtree};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut segtree = Segtree::<Max<i64>>::new(n);
    for i in 0..n {
        segtree.set(i, a[i]);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                    v: i64,
                }
                segtree.set(x - 1, v);
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let ans = segtree.prod((l - 1)..=(r - 1));
                println!("{}", ans);
            }
            3 => {
                input! {
                    x: usize,
                    v: i64,
                }
                let ans = segtree.max_right(x - 1, |val| *val < v);
                println!("{}", ans + 1);
            }
            _ => unreachable!(),
        }
    }
}
