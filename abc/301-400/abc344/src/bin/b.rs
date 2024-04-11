use proconio::input;

fn main() {
    let mut ans = vec![];
    loop {
        input! {
            a: usize,
        }
        ans.push(a);
        if a == 0 {
            break;
        }
    }
    ans.iter().rev().for_each(|i| println!("{}", i));
}
