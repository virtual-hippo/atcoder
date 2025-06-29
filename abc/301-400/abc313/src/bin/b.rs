use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let (g, h) = {
        let mut g = vec![vec![]; n];
        let mut h = vec![vec![]; n];
        for _ in 0..m {
            input! {
                a: usize,
                b: usize,
            }
            g[b - 1].push(a - 1);
            h[a - 1].push(b - 1);
        }
        (g, h)
    };
    let cnt_0 = (0..n).filter(|&i| g[i].len() == 0).count();

    if cnt_0 != 1 {
        println!("-1");
        return;
    }

    let saikyo_koho = (0..n).find(|&i| g[i].len() == 0).unwrap();
    let mut visited = vec![false; n];

    dfs(&h, saikyo_koho, &mut visited);
    if visited.iter().any(|&x| !x) {
        println!("-1");
    } else {
        println!("{}", saikyo_koho + 1);
    }
}

fn dfs(g: &[Vec<usize>], v: usize, visited: &mut [bool]) {
    visited[v] = true;
    for &u in &g[v] {
        if !visited[u] {
            dfs(g, u, visited);
        }
    }
}
