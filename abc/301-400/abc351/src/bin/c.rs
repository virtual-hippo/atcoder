use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut que = vec![];
    for i in 0..n {
        que.push(a[i]);
        while que.len() > 1 && que[que.len() - 1] == que[que.len() - 2] {
            que.pop();
            let val = que.pop().unwrap();
            que.push(val + 1)
        }
    }
    let ans = que.len();
    println!("{}", ans);
}
