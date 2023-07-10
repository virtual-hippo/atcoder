use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            p: [usize; n],
        }
        let ans = (0..n)
            .filter(|&i| i + 1 >= p[i])
            .filter(|&i| (0..n).filter(|&j| i < j && p[i] > p[j]).count() == 0)
            .count();
        println!("{}", ans);
    }
}
