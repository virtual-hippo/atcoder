use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let s = t.iter().sum::<usize>();
    let mut dp = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;
    for i in 1..n+1 {
        for j in 0..s+1 {
            if dp[i-1][j] {
                dp[i][j] = true;
            }
            if j < t[i-1] {
                //
            } else if dp[i-1][j-t[i-1]] {
                dp[i][j] = true;
            }
        }
    }
    let lower = if s % 2 == 1 {
        s/2 + 1
    } else {
        s/2
    };

    for i in lower..s+1 {
        if dp[n][i] {
            println!("{}", i);
            break;
        }
    }
}

