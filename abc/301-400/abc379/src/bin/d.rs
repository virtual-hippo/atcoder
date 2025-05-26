use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut que = VecDeque::new();
    let mut tt = 0;

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                que.push_back(tt);
            },
            2 => {
                input! {
                    t: usize,
                }
                tt += t;
            },
            3 => {
                input! {
                    h: usize,
                }
                let mut ans = 0;
                while let Some(v) = que.pop_front() {
                    let hh = tt - v;
                    if hh >= h {
                        ans += 1;
                        continue;
                    } else {
                        que.push_front(v);
                        break;
                    }
                }
                println!("{}", ans);
            },
            _ => {
                println!("Unknown query type: {}", query);
            },
        }
    }
}
