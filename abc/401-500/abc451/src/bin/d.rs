use itertools::*;
use proconio::input;

pub fn num_digits(x: u64) -> usize {
    if x == 0 {
        1
    } else {
        x.ilog10() as usize + 1
    }
}

fn dfs(nodes: &[u64], seen: &mut Vec<u64>, now: u64) {
    seen.push(now);
    for &node in nodes.iter() {
        let v = node + now * (10_u64.pow(num_digits(node) as u32));
        if v > 1_000_000_000 {
            continue;
        }
        dfs(nodes, seen, v);
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut x = 1;
    let mut xs = vec![];
    while x < 1_000_000_000 {
        xs.push(x);
        x = x << 1;
    }
    let mut seen = vec![];
    for &x in xs.iter() {
        dfs(&xs, &mut seen, x);
    }
    let candidate = seen.iter().copied().unique().sorted().collect_vec();

    let ans = candidate[n - 1];
    println!("{}", ans);
}
