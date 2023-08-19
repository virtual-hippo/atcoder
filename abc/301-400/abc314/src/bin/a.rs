use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let p = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".to_string();
    for (i, ch) in p.chars().enumerate() {
        print!("{}", ch);
        if i == n+1 {
            break;
        }
    }
}

