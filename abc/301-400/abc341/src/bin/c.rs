use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: String,
        s: [Chars; h],
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let mut flag = true;
                let mut current = (i, j);
                for ch in t.chars() {
                    match ch {
                        'L' => {
                            if current.1 > 0 && s[current.0][current.1 - 1] == '.' {
                                current.1 -= 1;
                            } else {
                                flag = false;
                                break;
                            }
                        }
                        'R' => {
                            if current.1 < w - 1 && s[current.0][current.1 + 1] == '.' {
                                current.1 += 1;
                            } else {
                                flag = false;
                                break;
                            }
                        }
                        'U' => {
                            if current.0 > 0 && s[current.0 - 1][current.1] == '.' {
                                current.0 -= 1;
                            } else {
                                flag = false;
                                break;
                            }
                        }
                        'D' => {
                            if current.0 < h - 1 && s[current.0 + 1][current.1] == '.' {
                                current.0 += 1;
                            } else {
                                flag = false;
                                break;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                if flag {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
