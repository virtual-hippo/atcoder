use proconio::{fastout, input};
use rustc_hash::FxHashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }

    let mut a = 0;
    let mut b = n;

    let mut set = FxHashSet::default();

    while (l[a] == 0 || l[b - 1] == 0) && a < b {
        set.insert(a);
        set.insert(b);
        if l[a] == 0 {
            a += 1;
        }
        if l[b - 1] == 0 {
            b -= 1;
        }
    }
    set.insert(a);
    set.insert(b);

    println!("{}", n + 1 - set.len());
}
