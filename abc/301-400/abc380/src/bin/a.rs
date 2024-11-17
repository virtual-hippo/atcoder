use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
    }
    let mut vec = vec![0; 11];
    for c in n.chars() {
        let i = (c as i32 - 48) as usize;
        vec[i] += 1;
    }
    let ans = vec[1] == 1 && vec[2] == 2 && vec[3] == 3;
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
