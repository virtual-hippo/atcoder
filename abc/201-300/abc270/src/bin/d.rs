use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let ans = solve(n, k, &a, 0, &mut vec![0; n + 1]);
    println!("{}", ans);
}

fn solve(n: usize, k: usize, a: &Vec<usize>, now: usize, memo: &mut Vec<usize>) -> usize {
    for i in 0..k {
        if a[i] <= now {
            memo[now] = memo[now].max(now - memo[now - a[i]]);
        }
    }

    if now == n {
        memo[n]
    } else {
        solve(n, k, a, now + 1, memo)
    }
}
