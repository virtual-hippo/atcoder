use proconio::input;
use std::collections::HashSet;

fn get_sorted_with_ind(src: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut ret = src
        .iter()
        .enumerate()
        .map(|(i, &v)| (i, v))
        .collect::<Vec<(usize, usize)>>();
    ret.sort_by(|(_, a), (_, b)| b.cmp(a));
    ret
}

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut set = HashSet::with_capacity(l);

    for _ in 0..l {
        input! {
            c: usize,
            d: usize,
        }
        set.insert((c - 1, d - 1));
    }
    let sorted_b = get_sorted_with_ind(&b);
    let mut ans = 0;
    for i in 0..n {
        for (j, val) in sorted_b.iter() {
            if set.contains(&(i, *j)) == false {
                ans = ans.max(a[i] + val);
                break;
            }
        }
    }
    println!("{}", ans);
}
