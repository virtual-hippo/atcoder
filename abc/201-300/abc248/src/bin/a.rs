use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut vec = vec![false; 10];
    for &ch in s.iter() {
        vec[ch as usize - 48] = true;
    }
    let ans = vec.iter().position(|&v|v==false).unwrap();
    println!("{}", ans);
}

