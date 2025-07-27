use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        s: [String; n],
    }

    let mut jisho = vec![];
    dfs(n, k, &s, &mut jisho, &mut vec![]);
    jisho.sort();

    println!("{}", jisho[x - 1]);
}

fn dfs(n: usize, k: usize, s: &[String], jisho: &mut Vec<String>, stack: &mut Vec<usize>) {
    if stack.len() == k {
        let mut t = String::new();
        for &i in stack.iter() {
            t.push_str(&s[i]);
        }
        jisho.push(t);
        return;
    }

    for i in 0..n {
        stack.push(i);
        dfs(n, k, s, jisho, stack);
        stack.pop();
    }
}
