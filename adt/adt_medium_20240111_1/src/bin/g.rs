use proconio::{fastout, input};

const R: usize = 0;
const B: usize = 1;
const N: usize = 2;

fn dfs(visited: &mut Vec<bool>, ropes: &Vec<[[usize; 2]; 2]>, pos: usize, color: usize) -> bool {
    if pos == ropes.len() {
        return false;
    }
    if visited[pos] {
        return true;
    }
    visited[pos] = true;
    return dfs(
        visited,
        ropes,
        ropes[pos][1 - color][0],
        ropes[pos][1 - color][1],
    );
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut ropes = Vec::with_capacity(n);
    for _ in 0..n {
        ropes.push([[n, N], [n, N]]);
    }
    for _ in 0..m {
        input! {
            a: usize,
            b: char,
            c: usize,
            d: char,
        }
        let a = a - 1;
        let c = c - 1;
        if b == 'R' {
            if d == 'R' {
                ropes[a][R] = [c, R];
                ropes[c][R] = [a, R];
            } else {
                ropes[a][R] = [c, B];
                ropes[c][B] = [a, R];
            }
        } else {
            if d == 'R' {
                ropes[a][B] = [c, R];
                ropes[c][R] = [a, B];
            } else {
                ropes[a][B] = [c, B];
                ropes[c][B] = [a, B];
            }
        }
    }
    let mut cnt = (0, 0);

    let mut visited = vec![false; n];
    for i in 0..n {
        if ropes[i][R][1] == N && ropes[i][B][1] == N {
            cnt.1 += 1;
            visited[i] = true;
            continue;
        }
        let color = if ropes[i][R][1] != N { B } else { R };
        if visited[i] {
            continue;
        }
        if dfs(&mut visited, &ropes, i, color) {
            cnt.0 += 1;
        } else {
            if ropes[i][B][1] != N && ropes[i][R][1] != N {
                visited[i] = false;
                dfs(&mut visited, &ropes, i, 1 - color);
            }
            cnt.1 += 1;
        }
    }
    println!("{} {}", cnt.0, cnt.1);
}
