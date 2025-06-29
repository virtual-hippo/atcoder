use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let (a, c) = if a < 0 { (a + 1_000_000_000, c + 1_000_000_000) } else { (a, c) };
    let (b, d) = if b < 0 { (b + 1_000_000_000, d + 1_000_000_000) } else { (b, d) };

    let ans = calc(c as u64, d as u64) + calc(a as u64, b as u64) - calc(a as u64, d as u64) - calc(c as u64, b as u64);
    println!("{}", ans);
}

fn calc(x: u64, y: u64) -> u64 {
    let xx = x / 4;
    let yy = y / 2;

    let mut ret = (4 * 2) * xx * yy;

    let mx = x % 4;
    let my = y % 2;

    //
    if mx == 1 {
        ret += 3 * yy;
    }
    if mx == 2 {
        ret += 6 * yy;
    }
    if mx == 3 {
        ret += 7 * yy;
    }

    if my == 1 {
        ret += 2 * 2 * xx;

        //
        if mx == 1 {
            ret += 2;
        }
        if mx == 2 {
            ret += 3;
        }
        if mx == 3 {
            ret += 3;
        }
    }

    ret
}
