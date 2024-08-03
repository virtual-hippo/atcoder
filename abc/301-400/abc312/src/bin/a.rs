use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    if ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"].contains(&s.as_str()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
