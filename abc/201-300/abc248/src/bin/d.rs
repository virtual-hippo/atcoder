use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let mut events = vec![vec![0]; n+1];

    for i in 0..n {
        events[a[i]].push(i+1);
    }

    for _ in 0..q {
        input! {
            (l, r, x): (usize, usize, usize),
        }

        let is_ok_l = |pos: usize| events[x][pos] < l;
        let result_l = binary_search((0, events[x].len()), is_ok_l);

        let is_ok_r = |pos: usize| events[x][pos] <= r;
        let result_r = binary_search((0, events[x].len()), is_ok_r);

        println!("{}", result_r.0 - result_l.0);
    }
}

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

