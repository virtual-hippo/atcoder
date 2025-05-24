use proconio::{fastout, input};

fn dfs(n: usize, t: usize, kankei: &Vec<usize>, teams: &mut Vec<usize>, next: usize) -> usize {
    if next == n {
        return if teams.len() == t { 1 } else { 0 };
    }

    let mut ans = 0;

    for i in 0..teams.len() {
        if teams[i] & kankei[next] != 0 {
            continue;
        };

        teams[i] |= 1 << next;
        ans += dfs(n, t, kankei, teams, next + 1);
        teams[i] ^= 1 << next;
    }

    if teams.len() < t {
        teams.push(1 << next);
        ans += dfs(n, t, kankei, teams, next + 1);
        teams.pop();
    }

    ans
}

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
    }
    let kankei = {
        let mut kankei = vec![0; n];
        for _ in 0..m {
            input! {
                a: usize,
                b: usize,
            }
            kankei[a - 1] |= 1 << (b - 1);
            kankei[b - 1] |= 1 << (a - 1);
        }
        kankei
    };

    let mut teams = vec![];
    let ans = dfs(n, t, &kankei, &mut teams, 0);

    println!("{}", ans);
}
