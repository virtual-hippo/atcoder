use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut vec = vec![];
    for (i, ch) in s.chars().enumerate() {
        if i <= 2 {
            continue;
        }

        let num: i32 = ch as i32 - 48;
        vec.push(num)
    }
    let v = vec[0] * 100 + vec[1] * 10 + vec[2];
    if v == 316 || v == 0 {
        println!("No");
        return;
    }
    if v < 350 {
        println!("Yes");
    } else {
        println!("No");
    }
}
