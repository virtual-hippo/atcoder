use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut flag = true;
    s.chars().for_each(|ch| {
        if ch == '|' {
            flag = !flag;
        } else if flag {
            print!("{}", ch);
        }
    });
}
