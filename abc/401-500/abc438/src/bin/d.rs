use ac_library::*;
use proconio::{fastout, input};

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

pub fn solve1() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
    }

    // 累積和
    let sa = generate_cumulative_sum(&a);
    let sb = generate_cumulative_sum(&b);
    let sc = generate_cumulative_sum(&c);

    let mut segtree = Segtree::<Max<i64>>::new(n + 1);
    for y in 0..n + 1 {
        segtree.set(y, sb[y] - sc[y]);
    }

    let mut ans = 0;
    for x in 1..n - 1 {
        let val0 = sa[x] + sc[n] - sb[x];
        let val1 = segtree.prod((x + 1)..n);
        ans = ans.max(val0 + val1);
    }

    println!("{}", ans);
}

pub fn solve2() {
    input! {
        n: usize,
    }

    let mut xs = vec![vec![0; 3]; n];

    for j in 0..3 {
        for i in 0..n {
            input! {
                x: i64,
            }
            xs[i][j] = x;
        }
    }

    let mut dp = vec![vec![i64::MIN; 3]; n];
    dp[0][0] = xs[0][0];

    for i in 1..n {
        for j in 0..3 {
            let pre = if j > 0 { dp[i - 1][j].max(dp[i - 1][j - 1]) } else { dp[i - 1][j] };
            dp[i][j] = pre + xs[i][j];
        }
    }

    let ans = dp[n - 1][2];
    println!("{}", ans);
}

#[fastout]
fn main() {
    solve1();
}
