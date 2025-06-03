use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut d = vec![vec![0; n]; n];

    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                w: usize,
            }
            d[i][j] = w;
            d[j][i] = w;
        }
    }

    _solve0(n, &d);
}

// bit DP
fn _solve0(n: usize, d: &Vec<Vec<usize>>) {
    let mut dp = vec![0; 1 << n];

    for bit in 0..((1 << n) - 1) {
        let l = (0..n).find(|&i| bit & (1 << i) == 0).unwrap();
        for i in (0..n).filter(|&i| bit & (1 << i) == 0) {
            let nbit = bit | (1 << l) | (1 << i);
            dp[nbit] = dp[nbit].max(dp[bit] + d[l][i]);
        }
    }

    let ans = (0..(1 << n)).map(|bit| dp[bit]).max().unwrap_or(0);
    println!("{}", ans);
}

// 再帰による全探索
fn _solve1(n: usize, d: &Vec<Vec<usize>>) {
    let mut ans = 0;
    _dfs(n, d, 0, &mut vec![], &mut ans);
    println!("{}", ans);
}

fn _dfs(n: usize, d: &Vec<Vec<usize>>, used: usize, pair: &mut Vec<(usize, usize)>, ans: &mut usize) {
    if pair.len() == n / 2 {
        let now = pair.iter().map(|&(i, j)| d[i][j]).sum::<usize>();
        *ans = (*ans).max(now);
        return;
    }

    for i in 0..(n - 1) {
        if used & (1 << i) == 0 {
            for j in (i + 1)..n {
                if used & (1 << j) == 0 {
                    pair.push((i, j));
                    _dfs(n, d, used | (1 << i) | (1 << j), pair, ans);
                    pair.pop();
                }
            }
        }
    }
}
