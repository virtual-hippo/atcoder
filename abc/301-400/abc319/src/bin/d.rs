use proconio::{fastout, input};

pub fn binary_search<F: Fn(usize) -> bool>(
    initial_pos: (usize, usize),
    is_ok: F,
) -> (usize, usize) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    (left, right)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: [usize; n],
    }
    let left = *(l.iter().max().unwrap()) - 1;
    let right = l.iter().sum::<usize>() + n - 1;

    let is_ok = |w: usize| {
        let mut length = 0;
        let mut rows = 1;
        for i in 0..n {
            if length > 0 {
                length += 1;
            }
            length += l[i];
            if length > w {
                length = l[i];
                rows += 1;
            }
        }
        m < rows
    };
    let ret = binary_search((left, right), is_ok);
    let ans = ret.1;
    println!("{}", ans);
}
