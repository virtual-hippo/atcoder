use proconio::{fastout, input, marker::Chars};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut t: Chars,
    }
    s.push('.');
    s.push('.');

    t.push('.');
    t.push('.');

    let mut queue = VecDeque::new();
    queue.push_back((s.clone(), n, 0));

    let mut dist = FxHashMap::default();
    dist.insert(s.clone(), 0_u64);

    while let Some((mut ss, k, d)) = queue.pop_front() {
        if ss == t {
            println!("{}", d);
            return;
        }

        for x in 0..n + 1 {
            if ss[x] == '.' || ss[x + 1] == '.' {
                continue;
            }

            ss[k] = ss[x];
            ss[k + 1] = ss[x + 1];
            ss[x] = '.';
            ss[x + 1] = '.';

            if !dist.contains_key(&ss) {
                dist.insert(ss.clone(), d + 1);
                queue.push_back((ss.clone(), x, d + 1));
            }

            ss[x] = ss[k];
            ss[x + 1] = ss[k + 1];
            ss[k] = '.';
            ss[k + 1] = '.';
        }
    }

    if let Some(ans) = dist.get(&t) {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}
