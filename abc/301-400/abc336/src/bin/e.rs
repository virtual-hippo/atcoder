use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ketasu = {
        let mut ret = 1;
        let mut nn = n;
        while nn / 10 > 0 {
            nn /= 10;
            ret += 1;
        }
        ret
    };
    if ketasu == 1 {
        println!("{}", n);
        return;
    }
    let ans1 = n / 3 - 3;
    let ans2 = {
        let mut ret = 0;
        let mut v = ketasu - 1;
        while v > 0 {
            ret += 9;
            v -= 1;
        }
        ret
    };
    let ans = ans1 + ans2 + 3;
    println!("{}", ans);
}
