use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut set = HashSet::new();
    let mut pigeon = (0..n).collect::<Vec<usize>>();
    let mut bx = vec![1; n];

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    p: usize,
                    h: usize,
                }
                let p = p - 1;
                let h = h - 1;

                bx[pigeon[p]] -= 1;
                if bx[pigeon[p]] <= 1 {
                    set.remove(&pigeon[p]);
                }

                pigeon[p] = h;

                bx[pigeon[p]] += 1;
                if bx[pigeon[p]] > 1 {
                    set.insert(pigeon[p]);
                }
            },
            2 => {
                println!("{}", set.len());
            },
            _ => unreachable!(),
        }
    }
}
