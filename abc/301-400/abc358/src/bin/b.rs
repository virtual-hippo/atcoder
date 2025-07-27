use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        t: [usize; n],
    }

    let mut queue = VecDeque::new();

    for i in 0..n {
        while let Some(&(time, _)) = queue.front() {
            if time + a <= t[i] {
                queue.pop_front();
                println!("{}", time + a);
                if queue.is_empty() {
                    break;
                }
                queue[0].0 = time + a;
            } else {
                break;
            }
        }

        queue.push_back((t[i], i));
    }

    while let Some(&(time, _)) = queue.front() {
        queue.pop_front();
        println!("{}", time + a);
        if queue.is_empty() {
            break;
        }
        queue[0].0 = time + a;
    }
}
