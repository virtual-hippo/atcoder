use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut memo = vec![true; 1 << n];

    if dfs(n, &s, 0, &mut memo) {
        println!("Yes");
        return;
    }

    println!("No");
}

fn dfs(n: usize, s: &[char], now: usize, memo: &mut [bool]) -> bool {
    if now > 0 && s[now - 1] == '1' {
        return false;
    }

    if memo[now] == false {
        return false;
    }

    if now == (1 << n) - 1 {
        return true;
    }

    for x in 0..n {
        if (now >> x) & 1 == 1 {
            continue;
        }
        if dfs(n, s, now | 1 << x, memo) {
            return true;
        }
    }

    memo[now] = false;
    false
}
