// 二部グラフか判定する
fn is_binary(graph: &Vec<Vec<usize>>, colors: &mut Vec<i8>, pos: usize, color: i8) -> bool {
    colors[pos] = color;
    for &point in graph[pos].iter() {
        if colors[point] == -1 {
            if !is_binary(graph, colors, point, 1 - color) {
                return false;
            }
        } else if colors[point] == color {
            return false;
        }
    }
    true
}

fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize) {
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point);
        }
    }
}
/// 単純無向グラフの連結成分を数える
///
/// * `n` - 頂点の数
/// * `m` - 辺の数
fn count_connected(n: usize, m: usize) -> usize {
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let (u, v): (usize, usize);
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut count_connected = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            dfs(&mut visited, &graph, i);
            count_connected += 1;
        }
    }
    count_connected
}

///  Count Simple Paths
///  頂点 1 を始点とする単純パス(同じ頂点を複数回通らないパス)の個数を数える
///  https://atcoder.jp/contests/abc284/tasks/abc284_e
fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize, count: &mut usize) {
    // if *count >= 1_000_000 {
    //     *count = 1_000_000;
    //     return
    // }
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            *count += 1;
            dfs(visited, graph, *point, count);
            visited[*point] = false;
        }
    }
}
