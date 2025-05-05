use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
    }

    let counter = {
        let mut counter = vec![0; 1_000_100];
        for _ in 0..n {
            input! {
                a: usize,
            }
            counter[a] += 1;
        }
        counter
    };

    if d == 0 {
        let ans = counter.iter().filter(|&&x| x > 0).map(|&x| x - 1).sum::<usize>();
        println!("{}", ans);
        return;
    }

    let ans = (0..d)
        .map(|m| {
            let counter = (m..1_000_100).step_by(d).map(|i| counter[i]).collect::<Vec<_>>();
            let mut dp = vec![[0; 2]; counter.len() + 1];

            for i in 0..dp.len() - 1 {
                dp[i + 1][0] = dp[i][1] + counter[i];
                dp[i + 1][1] = dp[i][0].max(dp[i][1]);
            }

            let sum = counter.iter().sum::<usize>();

            sum - (dp[dp.len() - 1][0]).max(dp[dp.len() - 1][1])
        })
        .sum::<usize>();

    println!("{}", ans);
}
