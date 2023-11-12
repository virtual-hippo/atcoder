use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=d[i - 1] {
            let curr = i.to_string() + j.to_string().as_str();
            let mut prev = ' ';
            let mut flag = true;
            for (i, ch) in curr.chars().enumerate() {
                if i != 0 {
                    if prev != ch {
                        flag = false;
                    }
                }
                prev = ch;
            }
            if flag {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
