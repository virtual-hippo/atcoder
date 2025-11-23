use proconio::{fastout, input, marker::Usize1};

fn dfs(to: &Vec<Vec<usize>>, root: usize, v: usize, p: usize, d: usize, ans: &mut Vec<(usize, usize)>) -> (usize, usize) {
    ans[v] = ans[v].max((d, root));
    let mut res = (d, v);
    for &u in to[v].iter().filter(|&&u| u != p) {
        res = res.max(dfs(to, root, u, v, d + 1, ans));
    }
    res
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut to = vec![vec![]; n];

    for _ in 0..n - 1 {
        input! {
            a: Usize1,
            b: Usize1,
        }
        to[a].push(b);
        to[b].push(a);
    }

    let mut ans = vec![(0, 0); n];
    let a = dfs(&to, 0, 0, usize::MAX, 0, &mut ans).1;
    let b = dfs(&to, a, a, usize::MAX, 0, &mut ans).1;
    dfs(&to, b, b, usize::MAX, 0, &mut ans);

    for v in 0..n {
        println!("{}", ans[v].1 + 1);
    }
}
