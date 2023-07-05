use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    const NUM: i64 = 998244353;
    let ans = if n % NUM == 0 {
        0
    } else if 0 <= n {
        n % 998244353
    } else {
        let neg_n = -n;
        NUM - (neg_n % 998244353)
    };
    println!("{}", ans);
}

