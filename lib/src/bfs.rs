use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<i64> {
    let mut dist = vec![-1_i64; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist[start] = 0;
    while queue.is_empty() == false {
        let pos = queue.pop_front().unwrap();
        for to in graph[pos].iter() {
            if dist[*to] == -1 {
                dist[*to] = dist[pos] + 1;
                queue.push_back(*to);
            }
        }
    }
    dist
}
