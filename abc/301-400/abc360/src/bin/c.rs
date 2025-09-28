use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        w: [usize; n],
    }

    let mut boxes = vec![vec![]; n];
    for i in 0..n {
        boxes[a[i] - 1].push(w[i]);
    }
    for i in 0..n {
        boxes[i].sort_by_key(|&w| std::cmp::Reverse(w));
    }

    let boxes = boxes.iter().filter(|b| b.len() > 1).collect::<Vec<_>>();

    let ans = boxes.iter().map(|b| b.iter().skip(1).sum::<usize>()).sum::<usize>();
    println!("{}", ans);
}
