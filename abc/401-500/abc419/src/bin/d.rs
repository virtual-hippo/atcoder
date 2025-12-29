use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        lr: [(usize, usize); m],
    }

    let mut g = vec![usize::MAX; 2 * n + 2];
    for i in 0..n {
        g[i] = i + 1;
        g[i + n + 1] = i + n + 1 + 1;
    }

    for (l, r) in lr {
        // let l = l - 1;
        // let r = r - 1;
        g.swap(l - 1, n + 1 + l - 1);
        g.swap(r, n + 1 + r);
    }

    let mut next = 0;
    for _ in 0..n {
        next = g[next];

        if next > n {
            print!("{}", t[next - n - 2]);
        } else {
            print!("{}", s[next - 1]);
        }
    }
}

// ------------------------------------------------------------------------------------------------
// 2025.12.28
// ------------------------------------------------------------------------------------------------

// 累積和
fn generate_cumulative_sum<T>(xs: &[T]) -> Vec<T>
where
    T: Copy + std::ops::AddAssign + From<i64>,
{
    std::iter::once(T::from(0))
        .chain(xs.iter().scan(T::from(0), |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .collect::<Vec<T>>()
}

#[fastout]
pub fn solve1() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        lr: [(Usize1,usize); m],
    }
    let mut a = vec![0_i64; n + 1];
    for i in 0..m {
        let (l, r) = lr[i];
        a[l] += 1;
        a[r] -= 1;
    }

    let sa = generate_cumulative_sum(&a);
    let mut ans = s.clone();
    for i in 0..n {
        if sa[i + 1] % 2 == 1 {
            ans[i] = t[i];
        }
    }

    let ans = ans.iter().copied().collect::<String>();
    println!("{}", ans);
}
