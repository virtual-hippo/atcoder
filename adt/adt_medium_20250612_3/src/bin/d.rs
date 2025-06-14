use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        t: String,
    }

    let mut now = (0, 0);
    let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut idx = 0;

    for ch in t.chars() {
        match ch {
            'S' => {
                let dir = dirs[idx];
                now.0 += dir.0;
                now.1 += dir.1;
            },
            'R' => {
                idx = (idx + 1) % 4;
            },

            _ => unreachable!(),
        }
    }

    println!("{} {}", now.0, now.1);
}
