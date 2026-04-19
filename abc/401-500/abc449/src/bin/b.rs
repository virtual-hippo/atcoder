use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut h = h;
    let mut w = w;

    for _ in 0..q {
        input! {
            q: usize,
            rc: usize,
        }
        if q == 1 {
            println!("{}", rc * w);
            h -= rc;
        } else {
            println!("{}", rc * h);
            w -= rc;
        }
    }
}
