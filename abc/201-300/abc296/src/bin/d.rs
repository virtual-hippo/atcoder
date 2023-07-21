use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut ans = std::usize::MAX;
    for a in 1..n+1 {
        let b = (m+a-1) / a;
        if b <= n {
            ans = ans.min(a * b);
        }
        if a > b {
            break;
        }
    }
    if ans == std::usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans)
    }
}

