use itertools::*;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    solve2();
}

pub fn solve0() {
    input! {
        _n: usize,
        t: Chars,
    }
    let a = t.iter().copied().map(|ch| 1 - (ch as u8 - '0' as u8)).collect_vec();

    // 累積和
    let s = std::iter::once(0_u8)
        .chain(a.iter().scan(0_u8, |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .map(|x| x % 2)
        .collect::<Vec<_>>();

    let ans = (0..2)
        .map(|i| {
            let cnt = s.iter().filter(|&&x| x == i).count();
            (cnt * (cnt - 1)) as u64 / 2
        })
        .sum::<u64>();

    println!("{}", ans);
}

pub fn solve1() {
    input! {
        n: usize,
        t: Chars,
    }
    let a = t.iter().copied().map(|ch| ch as u8 - '0' as u8).collect_vec();
    let mut dp = vec![vec![0_u64; 2]; n + 1];

    for i in 0..n {
        dp[i + 1][a[i] as usize] += 1;
        for j in 0..2 {
            let target = if a[i] == 0 { 1 - j } else { j };
            dp[i + 1][target] += dp[i][j];
        }
    }

    let ans = (1..=n).map(|i| dp[i][1]).sum::<u64>();
    println!("{}", ans);
}

pub fn solve2() {
    input! {
        n: usize,
        t: Chars,
    }
    let a = t.iter().copied().map(|ch| ch as u8 - '0' as u8).collect_vec();
    let mut cnt = vec![0_u64; 2];
    let mut ans = 0;

    for i in 0..n {
        if a[i] == 0 {
            cnt.swap(0, 1);
        }

        cnt[a[i] as usize] += 1;
        ans += cnt[1];
    }

    println!("{}", ans);
}
