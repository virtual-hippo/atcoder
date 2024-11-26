use ac_library::FenwickTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut bit = FenwickTree::new(n, 0);
    for p in 0..n {
        bit.add(p, a[p]);
    }

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            0 => {
                input! {
                    p: usize,
                    x: usize,
                }
                bit.add(p, x);
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", bit.sum(l..r));
            }
            _ => unreachable!(),
        }
    }
}
