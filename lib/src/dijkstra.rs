/// #ダイクストラ法
/// 最短経路を求める
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstra(graph: &Vec<Vec<(usize, i64)>>, n: usize) -> Vec<i64> {
    let mut heap = BinaryHeap::new();
    let mut kakutei = vec![false; n];
    let mut costs = vec![i64::MAX; n];
    costs[0] = 0;
    heap.push((Reverse(costs[0]), 0));
    while heap.is_empty() == false {
        let pos = heap.pop().unwrap().1;
        if kakutei[pos] {
            continue;
        }

        kakutei[pos] = true;
        for i in 0..graph[pos].len() {
            let next = graph[pos][i].0;
            let cost = graph[pos][i].1;
            if costs[next] > costs[pos] + cost {
                costs[next] = costs[pos] + cost;
                heap.push((Reverse(costs[next]), next));
            }
        }
    }

    return costs;
}
