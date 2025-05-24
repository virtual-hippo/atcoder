use proconio::{fastout, input};

fn dfs(n: usize, now: &mut Vec<char>) {
    if now.len() == n {
        println!("{}", now.iter().collect::<String>());
        return;
    }
    for &ch in &['a', 'b', 'c'] {
        now.push(ch);
        dfs(n, now);
        now.pop();
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    dfs(n, &mut vec![]);
}
