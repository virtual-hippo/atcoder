use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: u64,
    }

    let ans = (0..)
        .take_while(|&x| x * x <= d)
        .map(|x| {
            let yy = d - x.pow(2);
            let y0 = yy.isqrt();
            let y1 = y0 + 1;

            ((x.pow(2) + y0.pow(2)).abs_diff(d)).min((x.pow(2) + y1.pow(2)).abs_diff(d))
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
