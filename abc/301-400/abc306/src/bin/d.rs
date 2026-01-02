use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize,i64); n],
    }

    let dp = xy.iter().fold([Some(0_i64), None], |old, &(x, y)| match x {
        0 => {
            let ret = [old[0], old[0].map(|v| v + y), old[1].map(|v| v + y)]
                .into_iter()
                .flatten()
                .max();
            [ret, old[1]]
        },
        1 => {
            let ret = [old[1], old[0].map(|v| v + y)].into_iter().flatten().max();

            [old[0], ret]
        },
        _ => old,
    });

    let ans = dp[0].max(dp[1]).unwrap_or(0);
    println!("{}", ans);
}
