use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut current = 0;
    let mut day = 0;
    loop {
        day += 1;
        current += day;
        if current >= n {
            println!("{}", day);
            return;
        }
    }
}
