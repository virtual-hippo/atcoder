use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n],
    }
    let min_d = d.iter().min().unwrap();
    if p < min_d + q {
        println!("{}", p);
    } else {
        println!("{}", min_d + q);
    }
}

