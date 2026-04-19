use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if s.len() % 5 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
