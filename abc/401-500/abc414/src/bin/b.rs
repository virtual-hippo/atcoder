use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        cl: [(char, usize); n],
    }

    let ans = cl
        .into_iter()
        .map(|(c, l)| std::iter::repeat(c).take(l.min(101)))
        .flatten()
        .collect::<String>();

    if ans.len() > 100 {
        println!("Too Long");
    } else {
        println!("{}", ans);
    }
}
