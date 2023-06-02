// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

#[derive(Clone)]
struct Train {
    head: usize,
    tail: usize,
    next: usize,
    prev: usize,
    this: usize,
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut trains = vec![Train{
        head: 0,
        tail: 0,
        next: 0,
        prev: 0,
        this: 0,
    }; n+1];

    for i in 1..=n {
        trains[i].head = i+ 1;
        trains[i].tail = i+ 1;
        trains[i].this = i+ 1;
    }

    for _ in 0..q {
        input! {
            v: usize,
        }
        if v == 1 {
            input! {
                x: usize,
                y: usize,
            }
            trains[x].next = y;
            trains[x].tail = trains[y].tail;
            trains[y].prev = x;
            trains[y].head = trains[x].head;
        } else if v == 2 {
            input! {
                x: usize,
                y: usize,
            }
            trains[x].next = 0;
            trains[x].tail = x;
            trains[y].prev = 0;
            trains[y].head = y;
        } else if v == 3 {
            
            input! {
                x: usize,
            }
            let mut current = x;
            loop {
                if trains[current].prev == 0 {
                    break;
                } else {
                    current = trains[current].prev;
                }
            }
            let mut ans = vec![];
            while current != 0 {
                ans.push(current);
                current = trains[current].next;
            }
            print!("{} ", ans.len());
            for i in 0..ans.len() - 1 {
                print!("{} ", ans[i])
            }
            println!("{}", ans[ans.len() - 1])
        }
    }
}
