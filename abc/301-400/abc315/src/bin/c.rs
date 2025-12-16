use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        fs: [(usize,usize); n],
    }

    let fs = fs.iter().copied().sorted_by_key(|&(_, s)| std::cmp::Reverse(s)).collect_vec();

    let head = fs[0];

    let mut ans = 0;

    for i in 1..n {
        if fs[i].0 == head.0 {
            ans = ans.max(head.1 + fs[i].1 / 2);
            break;
        }
    }
    for i in 1..n {
        if fs[i].0 != head.0 {
            ans = ans.max(head.1 + fs[i].1);
            break;
        }
    }

    println!("{}", ans);
}
