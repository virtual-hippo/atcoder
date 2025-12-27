use proconio::{fastout, input, marker::Usize1};

// ダブリング
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
    }

    const D: usize = 30;

    let mut to = vec![vec![0; n]; D];
    let mut sum = vec![vec![0; n]; D];

    for i in 0..n {
        to[0][i] = a[i];
    }
    for i in 0..n {
        sum[0][i] = i + 1;
    }

    for j in 0..D - 1 {
        for i in 0..n {
            let x = to[j][i];
            to[j + 1][i] = to[j][x];
            sum[j + 1][i] = sum[j][i] + sum[j][x];
        }
    }

    for _ in 0..q {
        input! {
            t: usize,
            b: Usize1,
        }

        let (_, ans) = (0..D).fold(
            (b, 0),
            |(p, ans), j| {
                if t >> j & 1 == 1 {
                    (to[j][p], ans + sum[j][p])
                } else {
                    (p, ans)
                }
            },
        );
        println!("{}", ans);
    }
}
