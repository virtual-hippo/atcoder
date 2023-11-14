use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut ans = vec![];
    let mut current = n;
    while current > 1 {
        if current % 2 == 0 {
            current /= 2;
            ans.push('B');
        } else {
            current -= 1;
            ans.push('A');
            current /= 2;
            ans.push('B');
        }
    }
    ans.push('A');
    for ch in ans.iter().rev() {
        print!("{}", ch);
    }
}
