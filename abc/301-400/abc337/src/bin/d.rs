use proconio::marker::Chars;
use proconio::{fastout, input};

fn yoko(h: usize, w: usize, k: usize, s: &Vec<Vec<char>>) -> i64 {
    let mut ret = i64::MAX;
    if w < k {
        return ret;
    }
    for i in 0..h {
        let mut diff = vec![(0, 0); w];
        let mut ss = vec![(0, 0); w + 1];
        for j in 0..w {
            if s[i][j] == 'x' {
                diff[j] = (1, 0);
            } else if s[i][j] == '.' {
                diff[j] = (0, 1);
            }
        }
        for j in 0..w {
            ss[j + 1].0 = ss[j].0 + diff[j].0;
            ss[j + 1].1 = ss[j].1 + diff[j].1;
        }
        for j in 0..w - k + 1 {
            if ss[j + k].0 - ss[j].0 == 0 {
                ret = ret.min(ss[j + k].1 - ss[j].1);
            }
        }
    }
    ret
}

fn tate(h: usize, w: usize, k: usize, s: &Vec<Vec<char>>) -> i64 {
    let mut ret = i64::MAX;
    if h < k {
        return ret;
    }
    for j in 0..w {
        let mut diff = vec![(0, 0); h];
        let mut ss = vec![(0, 0); h + 1];
        for i in 0..h {
            if s[i][j] == 'x' {
                diff[i] = (1, 0);
            } else if s[i][j] == '.' {
                diff[i] = (0, 1);
            }
        }
        for i in 0..h {
            ss[i + 1].0 = ss[i].0 + diff[i].0;
            ss[i + 1].1 = ss[i].1 + diff[i].1;
        }
        for i in 0..h - k + 1 {
            if ss[i + k].0 - ss[i].0 == 0 {
                ret = ret.min(ss[i + k].1 - ss[i].1);
            }
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }
    let ret0 = yoko(h, w, k, &s);
    let ret1 = tate(h, w, k, &s);

    let ans = if ret0 == i64::MAX && ret1 == i64::MAX {
        -1
    } else if ret0 == i64::MAX && ret1 != i64::MAX {
        ret1
    } else if ret0 != i64::MAX && ret1 == i64::MAX {
        ret0
    } else {
        ret0.min(ret1)
    };
    println!("{}", ans);
}
