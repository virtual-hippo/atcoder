use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }
    let xs = (0..n).collect::<Vec<_>>();

    let ans = xs[0..l - 1]
        .iter()
        .chain(xs[l - 1..r].iter().rev())
        .chain(xs[r..n].iter())
        .collect::<Vec<_>>();

    for (i, &v) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v + 1);
    }

    println!("");
}
