use proconio::{fastout, input};

fn in_(a: (usize, usize), b: (usize, usize)) -> bool {
    !(b.1 <= a.0 || a.1 <= b.0)
}

#[fastout]
fn main() {
    input! {
        a: (usize,usize,usize,usize,usize,usize),
        b: (usize,usize,usize,usize,usize,usize),
    }

    let bound = in_((a.0, a.3), (b.0, b.3)) && in_((a.1, a.4), (b.1, b.4)) && in_((a.2, a.5), (b.2, b.5));

    if bound {
        println!("Yes");
    } else {
        println!("No");
    }
}
