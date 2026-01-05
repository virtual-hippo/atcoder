use proconio::{fastout, input, marker::*};

const DI: [i64; 4] = [-1, 0, 1, 0];
const DJ: [i64; 4] = [0, -1, 0, 1];

#[fastout]
fn main() {
    input! {
        h: i64,
        w: i64,
        r: Usize1,
        c: Usize1,
    }

    let ans = (0..4)
        .filter(|&i| {
            let r = r as i64 + DI[i];
            let c = c as i64 + DJ[i];

            0 <= r && r < h && 0 <= c && c < w
        })
        .count();
    println!("{}", ans);
}
