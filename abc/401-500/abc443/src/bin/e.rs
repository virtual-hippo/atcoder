use itertools::*;
use proconio::{input, marker::*};

fn solve() {
    input! {
        n: usize,
        c: Usize1,
        s: [Chars; n],
    }

    let bottom_walls = (0..n)
        .map(|j| (0..n).map(|i| n - 1 - i).find(|&i| s[i][j] == '#'))
        .collect_vec();

    let init = (
        (0..n).map(|j| j == c).collect_vec(),
        (0..n).map(|j| bottom_walls[j].is_none()).collect_vec(),
    );

    let (dp, _) = (0..n - 1).rev().fold(init, |(old, can_reach_b_wall), i| {
        let dp = (0..n)
            .filter(|&j| old[j.saturating_sub(1)] || old[j] || old[(j + 1).min(n - 1)])
            .fold(vec![false; n], |mut acc, nj| {
                acc[nj] = s[i][nj] == '.' || can_reach_b_wall[nj] || bottom_walls[nj] == Some(i);
                acc
            });

        let can_reach_b_wall = (0..n)
            .map(|j| can_reach_b_wall[j] || (dp[j] && (bottom_walls[j] == Some(i))))
            .collect_vec();

        (dp, can_reach_b_wall)
    });
    let ans = dp.iter().map(|&b| if b { '1' } else { '0' }).collect::<String>();
    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }

    (0..t).for_each(|_| solve());
}
