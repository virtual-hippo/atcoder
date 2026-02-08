use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let to = {
        let mut to = vec![vec![]; n];
        for i in 0..m {
            let (a, b) = ab[i];
            to[a].push(b);
        }
        to
    };

    let mut dsu = Dsu::new(n);
    let mut cnt = 0;
    let mut ans = vec![cnt];
    for a in (1..n).rev() {
        cnt += 1;
        for &b in to[a].iter() {
            if !dsu.same(a, b) {
                dsu.merge(a, b);
                cnt -= 1;
            }
        }
        ans.push(cnt);
    }

    let ans = ans.iter().copied().rev().collect_vec();
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
