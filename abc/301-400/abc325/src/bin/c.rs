use proconio::input;
use proconio::marker::Chars;

fn to_usize(u: usize, i: i64) -> usize {
    (u as i64 + i) as usize
}

fn dfs(visited: &mut Vec<Vec<bool>>, graph: &Vec<Vec<char>>, pos: (usize, usize)) {
    let dx = [-1, 0, 1];
    let dy = [-1, 0, 1];
    visited[pos.0][pos.1] = true;
    for &i in dx.iter() {
        for &j in dy.iter() {
            if (pos.0 == 0 && i == -1)
                || (pos.0 as usize == graph.len() - 1 && i == 1)
                || (pos.1 == 0 && j == -1)
                || (pos.1 as usize == graph[0].len() - 1 && j == 1)
            {
                continue;
            }
            let point = (to_usize(pos.0, i), to_usize(pos.1, j));
            if visited[point.0][point.1] {
                continue;
            }
            if graph[point.0][point.1] == '#' {
                dfs(visited, graph, point);
            }
        }
    }
}

fn main() {
    input! {
        (h,w): (usize, usize),
        s: [Chars; h],
    }
    let mut count_connected = 0;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' || visited[i][j] {
                continue;
            } else {
                dfs(&mut visited, &s, (i, j));
                count_connected += 1;
            }
        }
    }
    println!("{}", count_connected);
}
