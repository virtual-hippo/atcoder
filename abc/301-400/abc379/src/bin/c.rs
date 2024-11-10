use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [usize; m],
    }
    let mut xa = vec![(0, 1); m + 1];
    for i in 0..m {
        xa[i + 1].0 = x[i];
        xa[i + 1].1 = a[i];
    }
    xa.sort();

    let mut stone = xa[0].1;
    let mut ans = 0;

    for i in 0..m {
        let d_diff = xa[i + 1].0 - xa[i].0;
        if stone < d_diff {
            println!("-1");
            return;
        }

        ans += (d_diff * (2 * (stone - 1) - (d_diff - 1))) / 2;
        stone += xa[i + 1].1;
        stone -= d_diff;
    }

    if xa[m].0 == n {
        if stone == 1 {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    } else {
        let d_diff = n - xa[m].0;
        if stone < d_diff {
            println!("-1");
            return;
        }
        ans += (d_diff * (2 * (stone - 1) - (d_diff - 1))) / 2;
        stone -= d_diff;
        if stone == 1 {
            println!("{}", ans);
            return;
        } else {
            println!("-1");
        }
    }
}
