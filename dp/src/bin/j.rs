use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }
    let mut dp = vec![vec![vec![-1.0; 310]; 310]; 310];
    let mut counter = [0; 3];

    for i in 0..n {
        if a[i] == 1.0 {
            counter[0] += 1;
        } else if a[i] == 2.0 {
            counter[1] += 1;
        } else if a[i] == 3.0 {
            counter[2] += 1;
        }
    }
    let ans = solve(counter[0], counter[1], counter[2], &mut dp, &mut counter, n);
    println!("{:.10}", ans);
}

fn solve(i: usize, j: usize, k: usize, dp: &mut Vec<Vec<Vec<f64>>>, counter: &mut [usize; 3], n: usize) -> f64 {
    if dp[i][j][k] >= 0.0 {
        return dp[i][j][k];
    }
    if i == 0 && j == 0 && k == 0 {
        return 0.0;
    }

    let mut res = 0.0;
    if i > 0 {
        res += solve(i - 1, j, k, dp, counter, n) * i as f64;
    }
    if j > 0 {
        res += solve(i + 1, j - 1, k, dp, counter, n) * j as f64;
    }
    if k > 0 {
        res += solve(i, j + 1, k - 1, dp, counter, n) * k as f64;
    }
    res += n as f64;
    res *= 1.0 / (i + j + k) as f64;

    dp[i][j][k] = res;
    res
}
