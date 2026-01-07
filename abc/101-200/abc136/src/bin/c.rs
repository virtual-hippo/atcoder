use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let ans = h.iter().copied().rev().fold((true, i64::MAX), |(acc, pre), x| {
        if pre >= x {
            (acc && pre >= x, x)
        } else {
            let x = x - 1;
            (acc && pre >= x, x)
        }
    });
    if ans.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
