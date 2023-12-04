use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }
    let mut ans = 0;
    for i in 0..n {
        let mut flag = false;
        for a in 1..1000 {
            for b in 1..1000 {
                if 4 * a * b + 3 * a + 3 * b == s[i] {
                    flag = true;
                    break;
                }
            }
        }
        if flag == false {
            ans += 1;
        }
    }
    println!("{}", ans);
}
