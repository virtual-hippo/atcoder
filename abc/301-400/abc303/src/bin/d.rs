use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }

    let init = if s[0] == 'a' { [x, z + y] } else { [y, z + x] };

    let dp = (1..s.len()).fold(init, |old, i| {
        if s[i] == 'a' {
            [(old[0] + x).min(old[1] + z + x), (old[1] + y).min(old[0] + z + y)]
        } else {
            [(old[0] + y).min(old[1] + z + y), (old[1] + x).min(old[0] + z + x)]
        }
    });

    let ans = dp[0].min(dp[1]);
    println!("{}", ans);
}
