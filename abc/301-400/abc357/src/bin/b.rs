use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let n = s.len();
    let cnt = s.chars().filter(|&c| c.is_ascii_uppercase()).count();
    let cnt1 = n - cnt;
    let ans = if cnt > cnt1 {
        s.to_uppercase().chars().collect::<String>()
    } else {
        s.to_lowercase().chars().collect::<String>()
    };

    println!("{}", ans);
}
