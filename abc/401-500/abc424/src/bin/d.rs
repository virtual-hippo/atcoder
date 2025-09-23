use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut s = s;
    let ans = rec(0, 0, h, w, &mut s, 0, &mut 10);

    println!("{}", ans);
}

fn rec(i: usize, j: usize, h: usize, w: usize, s: &mut Vec<Vec<char>>, now: usize, ans: &mut usize) -> usize {
    if now > 9 || now > *ans {
        return *ans;
    }

    if !is_all_black(i, j, i + 1, j + 1, s) {
        return next(i, j, h, w, s, now, ans);
    }

    let mut res = usize::MAX;

    // for (di, dj) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
    for (di, dj) in [(1, 0), (1, 1)] {
        s[i + di][j + dj] = '.';
        let now_res = next(i, j, h, w, s, now + 1, ans);
        s[i + di][j + dj] = '#';
        res = res.min(now_res);
    }
    res
}

fn next(i: usize, j: usize, h: usize, w: usize, s: &mut Vec<Vec<char>>, now: usize, ans: &mut usize) -> usize {
    if j + 1 < w - 1 {
        rec(i, j + 1, h, w, s, now, ans)
    } else if i + 1 < h - 1 {
        rec(i + 1, 0, h, w, s, now, ans)
    } else {
        *ans = (*ans).min(now);
        *ans
    }
}

fn is_all_black(i0: usize, j0: usize, i1: usize, j1: usize, s: &Vec<Vec<char>>) -> bool {
    s[i0][j0] == '#' && s[i0][j1] == '#' && s[i1][j0] == '#' && s[i1][j1] == '#'
}
