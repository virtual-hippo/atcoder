use proconio::{input, marker::*};

fn dfs(
    to: &[Vec<usize>],
    w: usize,
    s: &[Vec<char>],
    seen: &mut Vec<Vec<bool>>,
    finished: &mut Vec<Vec<bool>>,
    day: usize,
    u: usize,
) -> bool {
    seen[u][day] = true;

    let next_day = (day + 1) % w;

    for &v in to[u].iter() {
        if s[v][next_day] != 'o' {
            continue;
        }
        if finished[v][next_day] {
            continue;
        }
        if seen[v][next_day] && !finished[v][next_day] {
            return true;
        }
        if dfs(to, w, s, seen, finished, next_day, v) {
            return true;
        }
    }
    finished[u][day] = true;
    false
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
        w: usize,
        s: [Chars;n],
    }

    let to = {
        let mut to = vec![vec![]; n];
        for u in 0..n {
            to[u].push(u);
        }
        for &(u, v) in uv.iter() {
            to[u].push(v);
            to[v].push(u);
        }
        to
    };

    let mut seen = vec![vec![false; w]; n];
    let mut finished = vec![vec![false; w]; n];
    let mut ans = false;
    for u in 0..n {
        if s[u][0] != 'o' {
            continue;
        }
        if finished[u][0] {
            continue;
        }
        ans |= dfs(&to, w, &s, &mut seen, &mut finished, 0, u);
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}
