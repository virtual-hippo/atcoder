use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut cnt_tree = FenwickTree::new(5 * 100_100, 0_i64);
    let mut sum_tree = FenwickTree::new(5 * 100_100, 0_i64);

    for i in 0..n {
        cnt_tree.add(a[i] as usize, 1);
        sum_tree.add(a[i] as usize, a[i] as i64);
    }

    let mut a = a;

    for _ in 0..q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: Usize1,
                y: usize,
            }

            cnt_tree.add(a[x], -1);
            sum_tree.add(a[x], -(a[x] as i64));

            a[x] = y;

            cnt_tree.add(a[x], 1);
            sum_tree.add(a[x], a[x] as i64);
        }
        if q == 2 {
            input! {
                l: usize,
                r: usize,
            }

            let ans = if r <= l {
                l * (n)
            } else {
                // l < r
                let count0 = cnt_tree.sum(..=l) as usize;
                let v0 = l * count0;

                let count1 = cnt_tree.sum(r..) as usize;
                let v1 = r * count1;

                let v2 = sum_tree.sum((l + 1)..(r)) as usize;

                v0 + v1 + v2
            };

            println!("{}", ans);
        }
    }
}
