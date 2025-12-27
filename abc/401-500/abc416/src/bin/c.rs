use proconio::{fastout, input, marker::Usize1};

fn dfs(n: usize, k: usize, a: &mut Vec<usize>, m: &mut Vec<String>, s: &Vec<String>) {
    if a.len() == k {
        m.push(a.iter().map(|&i| s[i].chars()).flatten().collect::<String>());
        return;
    }

    for i in 0..n {
        a.push(i);
        dfs(n, k, a, m, s);
        a.pop();
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: Usize1,
        s: [String; n],
    }

    let mut m = vec![];
    dfs(n, k, &mut vec![], &mut m, &s);

    m.sort();

    println!("{}", m[x]);
}
