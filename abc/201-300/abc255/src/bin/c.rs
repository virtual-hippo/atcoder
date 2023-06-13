use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }
    let diff = std::cmp::max(x - a, a - x);
    if d == 0 {
        println!("{}", diff);
        return;
    }
    let (min, max) = if d > 0 {
        (a, a + (n-1) * d)
    } else {
        (a + (n-1) * d, a)
    };
    let ans = if x < min {
        min - x
    } else if max < x {
        x - max
    } else {
        let abs_d = std::cmp::max(-d, d);
        let rem = diff % abs_d;
        std::cmp::min(rem, abs_d - rem)
    };
    println!("{}", ans);
}
