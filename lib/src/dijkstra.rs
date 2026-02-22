/// ダイクストラ法
/// 重み付きグラフの単一始点最短経路を求める
/// graph[v] = [(next, cost), ...] の隣接リスト形式
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: &[Vec<(usize, i64)>], start: usize) -> Vec<i64> {
    let n = graph.len();
    let mut heap = BinaryHeap::new();
    let mut visited = vec![false; n];
    let mut costs = vec![i64::MAX; n];
    costs[start] = 0;
    heap.push((Reverse(0i64), start));

    while let Some((Reverse(cost), pos)) = heap.pop() {
        if visited[pos] {
            continue;
        }
        visited[pos] = true;

        for &(next, edge_cost) in &graph[pos] {
            let next_cost = cost + edge_cost;
            if costs[next] > next_cost {
                costs[next] = next_cost;
                heap.push((Reverse(next_cost), next));
            }
        }
    }

    costs
}
