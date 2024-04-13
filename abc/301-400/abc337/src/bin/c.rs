use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let start = a.iter().position(|&v| v == -1).unwrap();
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        if a[i] == -1 {
            continue;
        }
        graph[a[i] as usize - 1].push(i);
    }
    let mut ans = vec![start];
    loop {
        let tail = ans[ans.len() - 1];
        if graph[tail].len() == 0 {
            break;
        }
        ans.push(graph[tail][0]);
    }
    for i in 0..ans.len() {
        print!("{} ", ans[i] + 1);
    }
}
