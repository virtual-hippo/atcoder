use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [char; 3],
    }
    if s[0] == '<' && s[1] == '<' && s[2] == '<' {
        println!("B");
    } else if s[0] == '<' && s[1] == '>' && s[2] == '>' {
        println!("A");
    } else if s[0] == '<' && s[1] == '<' && s[2] == '>' {
        println!("C");
    } else if s[0] == '>' && s[1] == '>' && s[2] == '>' {
        println!("B");
    } else if s[0] == '>' && s[1] == '<' && s[2] == '<' {
        println!("A");
    } else if s[0] == '>' && s[1] == '>' && s[2] == '<' {
        println!("C");
    }
}
