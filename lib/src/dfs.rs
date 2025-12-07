// 二部グラフか判定する
pub fn is_binary(graph: &Vec<Vec<usize>>, colors: &mut Vec<i8>, pos: usize, color: i8) -> bool {
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

pub fn dfs(visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>, pos: usize) {
    visited[pos] = true;
    for point in graph[pos].iter() {
        if visited[*point] == false {
            dfs(visited, graph, *point);
        }
    }
}
