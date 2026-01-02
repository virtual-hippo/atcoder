use proconio::{fastout, input, marker::Chars};

fn func1(n: usize, s: &Vec<char>, c: &Vec<u64>, m: usize) -> Vec<u64> {
    let mut s_cloned = s.clone();
    let mut cost = 0;
    let mut ret = vec![0; n];
    for i in 0..n {
        if i % 2 == m && s[i] == '1' {
            s_cloned[i] = '0';
            cost += c[i]
        }
        if i % 2 != m && s[i] == '0' {
            s_cloned[i] = '1';
            cost += c[i]
        }
        ret[i] = cost;
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [u64; n],
    }

    let costs1 = func1(n, &s, &c, 1);
    let costs2 = func1(n, &s, &c, 0);

    let mut ans = u64::MAX;
    for i in 0..n - 1 {
        let current1 = costs1[i] + (costs2[n - 1] - costs2[i]);
        let current2 = costs2[i] + (costs1[n - 1] - costs1[i]);
        ans = ans.min(current1.min(current2));
    }

    println!("{}", ans);
}

// https://publish.obsidian.md/naoya/articles/%E3%80%8C%E3%83%95%E3%82%A7%E3%83%BC%E3%82%BA%E3%80%8D%E3%82%92%E7%8A%B6%E6%85%8B%E3%81%AB%E3%81%99%E3%82%8BDP
pub fn solve() {
    input! {
        n: usize,
        s: String,
        c: [usize; n],
    }
    let s = s.chars().map(|c| (c as u8 - b'0') as usize).collect_vec();

    let mut init = [[usize::MAX; 2]; 2];
    init[0][s[0]] = 0;
    init[0][s[0] ^ 1] = c[0];

    let dp = (1..n).fold(init, |old, i| {
        let mut dp = [[usize::MAX; 2]; 2];

        let cost = |to: usize| -> usize {
            if s[i] == to {
                0
            } else {
                c[i]
            }
        };

        // before -> before (0 -> 1 or 1 -> 0)
        for to in 0..2 {
            if old[0][to ^ 1] != usize::MAX {
                dp[0][to] = old[0][to ^ 1] + cost(to);
            }
        }

        // before -> after (0 -> 0 or 1 -> 1)
        for to in 0..2 {
            if old[0][to] != usize::MAX {
                dp[1][to] = dp[1][to].min(old[0][to] + cost(to));
            }
        }

        // after -> after (0 -> 1 or 1 -> 0)
        for to in 0..2 {
            if old[1][to ^ 1] != usize::MAX {
                dp[1][to] = dp[1][to].min(old[1][to ^ 1] + cost(to));
            }
        }

        dp
    });

    let ans = dp[1][0].min(dp[1][1]);
    println!("{}", ans);
}
