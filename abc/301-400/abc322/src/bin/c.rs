use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    a.sort_by(|a, b| b.cmp(a));
    for i in 0..n {
        println!("{}", a[a.len() - 1] - (i + 1));
        if a.len() > 0 && i + 1 == a[a.len() - 1] {
            a.pop();
        }
    }
}
