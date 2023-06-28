use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (h,w): (usize, usize),
        (x,y): (usize, usize),
        s: [Chars; h],
    }
    let (x,y) = (x-1,y-1);
    let up = {
        let mut ans = 0;
        let mut current = x;
        while current > 0 {
            current -= 1;
            if s[current][y] == '#' {
                break;
            } else {
                ans += 1;
            }
        }
        ans
    };
    let bottom = {
        let mut ans = 0;
        let mut current = x;
        while current < h - 1 {
            current += 1;
            if s[current][y] == '#' {
                break;
            } else {
                ans += 1;
            }
        }
        ans
    };
    let left = {
        let mut ans = 0;
        let mut current = y;
        while current > 0 {
            current -= 1;
            if s[x][current] == '#' {
                break;
            } else {
                ans += 1;
            }
        }
        ans
    };
    let right = {
        let mut ans = 0;
        let mut current = y;
        while current < w - 1 {
            current += 1;
            if s[x][current] == '#' {
                break;
            } else {
                ans += 1;
            }
        }
        ans
    };
    println!("{}", up + bottom + left + right + 1);
}

