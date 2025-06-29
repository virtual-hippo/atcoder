use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mx = p.iter().max().unwrap();

    if p[0] > *mx {
        println!("0");
        return;
    }

    if p.iter().filter(|&&x| x == *mx).count() == 1 && p[0] == *mx {
        println!("0");
        return;
    }

    let ans = mx.saturating_sub(p[0]) + 1;
    println!("{}", ans);
}
