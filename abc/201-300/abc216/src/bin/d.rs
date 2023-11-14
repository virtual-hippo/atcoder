use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![];
    let mut mem = vec![vec![]; n + 1];
    for i in 0..m {
        input! {
            k: usize,
            aa: [usize; k],
        }
        a.push(VecDeque::from(aa));
        mem[a[i][0]].push(i);
    }

    let mut deque = VecDeque::new();
    for i in 1..=n {
        if mem[i].len() == 2 {
            deque.push_back(i);
        }
    }

    while !deque.is_empty() {
        let now = deque.pop_front().unwrap();
        let (p, q) = (mem[now][0], mem[now][1]);
        for pp in [p, q] {
            a[pp].pop_front();
            if !a[pp].is_empty() {
                let a_head = a[pp][0];
                mem[a_head].push(pp);
                if mem[a_head].len() == 2 {
                    deque.push_back(a_head);
                }
            }
        }
    }
    for i in 0..m {
        if !a[i].is_empty() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
