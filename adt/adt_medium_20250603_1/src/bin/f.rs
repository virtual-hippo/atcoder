use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
    }
    let mut red = vec![0_u64; n + 1];
    let mut blue = vec![0_u64; n + 1];
    red[n] = 1;

    for i in (2..n + 1).rev() {
        if red[i] > 0 {
            red[i - 1] += red[i];
            blue[i] += x * red[i];
            red[i] = 0;
        }

        if blue[i] > 0 {
            red[i - 1] += blue[i];
            blue[i - 1] += y * blue[i];
            blue[i] = 0;
        }
    }

    let ans = blue[1];
    println!("{}", ans);
}
