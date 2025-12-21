use ac_library::{Max, Segtree};
use proconio::{fastout, input, marker::Usize1};

fn get_val(j: usize, x: i64, y: i64) -> i64 {
    let mut val = 0;
    if j & 1 == 0 {
        val += x;
    } else {
        val -= x;
    }
    if j & 2 == 0 {
        val += y;
    } else {
        val -= y;
    }

    val
}

fn update(segtrees: &mut [Segtree<Max<i64>>], i: usize, x: i64, y: i64) {
    for j in 0..4 {
        segtrees[j].set(i, get_val(j, x, y));
    }
}

fn calc(segtrees: &[Segtree<Max<i64>>], l: usize, r: usize, x: i64, y: i64) -> i64 {
    (0..4).map(|j| segtrees[j].prod(l..r) - get_val(j, x, y)).max().unwrap_or(0)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut segtrees = vec![
        Segtree::<Max<i64>>::new(n),
        Segtree::<Max<i64>>::new(n),
        Segtree::<Max<i64>>::new(n),
        Segtree::<Max<i64>>::new(n),
    ];

    for i in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        update(&mut segtrees, i, x, y);
    }

    for _ in 0..q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {
                    i: Usize1,
                    x: i64,
                    y: i64,
                }

                update(&mut segtrees, i, x, y);
            },
            2 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: i64,
                    y: i64,
                }
                let ans = calc(&segtrees, l, r, x, y);
                println!("{}", ans);
            },
            _ => unreachable!(),
        }
    }
}
