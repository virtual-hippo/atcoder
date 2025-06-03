use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: (i64,i64),
        b: (i64,i64),
        c: (i64,i64),
        d: (i64,i64),
    }

    let is_ok = calc_naiseki_from_pos(a, b, c) >= 0
        && calc_naiseki_from_pos(b, c, d) >= 0
        && calc_naiseki_from_pos(c, d, a) >= 0
        && calc_naiseki_from_pos(d, a, b) >= 0;

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn calc_naiseki_from_pos(x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> i64 {
    let a = (y.0 - x.0, y.1 - x.1);
    let b = (z.0 - y.0, z.1 - y.1);

    a.0 * b.1 - a.1 * b.0
}
