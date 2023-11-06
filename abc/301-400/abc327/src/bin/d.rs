use proconio::input;

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

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        b: [usize; m],
    }
    let mut graph = vec![vec![]; n + 1];
    for i in 0..m {
        graph[a[i]].push(b[i]);
        graph[b[i]].push(a[i]);
    }
    let mut x = vec![-1; n + 1];
    for v in 1..n + 1 {
        if x[v] != -1 {
            continue;
        }
        if !is_binary(&graph, &mut x, v, 0) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
