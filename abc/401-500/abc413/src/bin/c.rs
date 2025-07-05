use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut a = VecDeque::new();

    for _ in 0..q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {
                    c: u64,
                    x: u64,
                }
                a.push_back((x, c));
            },
            2 => {
                input! {
                    k: u64,
                }
                let mut cnt = 0;
                let mut s = 0;
                while cnt < k {
                    if let Some((x, c)) = a.pop_front() {
                        if cnt + c <= k {
                            cnt += c;
                            s += x * c;
                        } else {
                            a.push_front((x, c - (k - cnt)));
                            s += x * (k - cnt);
                            cnt = k;
                        }
                    } else {
                        break;
                    }
                }

                println!("{}", s);
            },
            _ => unreachable!(),
        }
    }
}
