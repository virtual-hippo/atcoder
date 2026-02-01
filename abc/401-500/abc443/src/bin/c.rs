use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        mut a: [usize; n],
    }
    a.push(t);
    let a = a;

    let (_, ans) = a.iter().fold((0, 0), |acc, &a| {
        let (open_time, ans) = acc;
        if open_time < a {
            (a + 100, ans + a - open_time)
        } else {
            acc
        }
    });

    println!("{}", ans);
}
