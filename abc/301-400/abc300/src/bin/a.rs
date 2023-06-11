use proconio::input;

fn main() {
    input! {
        (n,a,b): (usize, usize, usize),
        c: [usize; n],
    }
    let ans = a+ b;
    for i in 0..n {
        if ans == c[i] {
            println!("{}", i+1);
            return;
        }
    }
}
